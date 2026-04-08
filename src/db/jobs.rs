use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::utils::with_tenant;
use crate::{errors::ApiError, models::Job};
use futures_util::FutureExt;

// CREATE JOB
pub async fn create_job(
    pool: &PgPool,
    tenant_id: Uuid,
    job_type: String,
    payload: Value,
) -> Result<Job, ApiError> {
    let job = with_tenant(pool, tenant_id, |tx| {
        async move {
            let job = sqlx::query_as!(
                Job,
                r#"
            INSERT INTO jobs (tenant_id, job_type, payload)
            VALUES ($1, $2, $3)
            RETURNING
                id, tenant_id, job_type, payload,
                status, attempts, max_attempts,
                error_message, created_at, updated_at
            "#,
                tenant_id,
                job_type,
                payload
            )
            .fetch_one(&mut **tx)
            .await?;

            Ok(job)
        }
        .boxed()
    })
    .await?;

    Ok(job)
}

// GET PENDING JOBS (admin/debug)

pub async fn list_pending_jobs(pool: &PgPool, tenant_id: Uuid) -> Result<Vec<Job>, ApiError> {
    let jobs = with_tenant(pool, tenant_id, |tx| {
        async move {
            let jobs = sqlx::query_as!(
                Job,
                r#"
            SELECT *
            FROM jobs
            WHERE status = 'pending'
            ORDER BY created_at ASC
            "#
            )
            .fetch_all(&mut **tx)
            .await?;

            Ok(jobs)
        }
        .boxed()
    })
    .await?;

    Ok(jobs)
}
