use sqlx::PgPool;
use tokio::time::{sleep, Duration};

pub async fn job_worker(pool: PgPool) {
    loop {
        let mut tx = match pool.begin().await {
            Ok(tx) => tx,
            Err(_) => {
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };

        // Bypass RLS for this transaction
        if let Err(e) = sqlx::query("SET LOCAL app.bypass_rls = 'on'")
            .execute(&mut *tx)
            .await
        {
            eprintln!("Worker: failed to set bypass_rls: {}", e);
            let _ = tx.rollback().await;
            sleep(Duration::from_secs(1)).await;
            continue;
        }

        let job = sqlx::query!(
            r#"
            SELECT id, job_type, payload
            FROM jobs
            WHERE status = 'pending'
            ORDER BY created_at
            LIMIT 1
            FOR UPDATE SKIP LOCKED
            "#
        )
        .fetch_optional(&mut *tx)
        .await;

        match job {
            Ok(Some(job)) => {
                println!("Processing job: {:?}", job.id);

                // simulate processing
                let result = process_job(&job.job_type, &job.payload).await;

                match result {
                    Ok(_) => {
                        let _ = sqlx::query!(
                            "UPDATE jobs SET status = 'completed' WHERE id = $1",
                            job.id
                        )
                        .execute(&mut *tx)
                        .await;
                    }
                    Err(e) => {
                        let _ = sqlx::query!(
                            r#"
                            UPDATE jobs
                            SET attempts = attempts + 1,
                                error_message = $2,
                                status = CASE
                                    WHEN attempts + 1 >= max_attempts THEN 'failed'
                                    ELSE 'pending'
                                END
                            WHERE id = $1
                            "#,
                            job.id,
                            e.to_string()
                        )
                        .execute(&mut *tx)
                        .await;
                    }
                }

                if let Err(e) = tx.commit().await {
                    eprintln!("Worker: failed to commit job {}: {}", job.id, e);
                }
            }
            Ok(None) => {
                let _ = tx.rollback().await;
                sleep(Duration::from_secs(1)).await;
            }
            Err(e) => {
                eprintln!("Worker: query error: {}", e);
                let _ = tx.rollback().await;
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn process_job(
    job_type: &str,
    payload: &serde_json::Value,
) -> Result<(), String> {
    println!("Executing job type: {} payload: {}", job_type, payload);

    // simulate success
    Ok(())
}