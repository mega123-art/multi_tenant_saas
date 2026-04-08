use sqlx::PgPool;
use uuid::Uuid;
use serde_json::Value;
use futures_util::FutureExt;

use crate::models::Task;
use crate::errors::ApiError;
use crate::db::utils::with_tenant;


//CREATE TASK 


pub async fn create_task(
    pool: &PgPool,
    tenant_id: Uuid,
    project_id: Uuid,
    parent_task_id: Option<Uuid>,
    title: String,
    description: Option<String>,
    metadata: Value,
) -> Result<Task, ApiError> {
    let task = with_tenant(pool, tenant_id, |tx| async move {
        let task = sqlx::query_as!(
            Task,
            r#"
            INSERT INTO tasks (
                tenant_id, project_id, parent_task_id,
                title, description, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id, tenant_id, project_id, parent_task_id,
                title, description, status, priority,
                metadata, version, created_at, updated_at
            "#,
            tenant_id,
            project_id,
            parent_task_id,
            title,
            description,
            metadata
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(task)
    }.boxed())
    .await?;

    Ok(task)
}


// LIST TASKS (basic)


pub async fn list_tasks(
    pool: &PgPool,
    tenant_id: Uuid,
    project_id: Option<Uuid>,
    search: Option<String>,
    status: Option<String>,
    label: Option<String>,
) -> Result<Vec<Task>, ApiError> {
    let tasks = with_tenant(pool, tenant_id, |tx| {
        async move {
            let mut base = String::from(
                r#"
                SELECT
                    id, tenant_id, project_id, parent_task_id,
                    title, description, status, priority,
                    metadata, version, created_at, updated_at
                FROM tasks
                WHERE 1=1
                "#
            );

            let mut i = 1;

            // Step 1: Build SQL string dynamically
            if project_id.is_some() {
                base.push_str(&format!(" AND project_id = ${}", i));
                i += 1;
            }

            if search.is_some() {
                base.push_str(&format!(
                    " AND search_vector @@ plainto_tsquery('english', ${})",
                    i
                ));
                i += 1;
            }

            if status.is_some() {
                base.push_str(&format!(" AND status = ${}", i));
                i += 1;
            }

            if label.is_some() {
                base.push_str(&format!(" AND metadata @> ${}", i));
                i += 1;
            }

            base.push_str(" ORDER BY created_at DESC");

            // Step 2: Build final query and bind ALL parameters
            let mut query = sqlx::query_as::<_, Task>(&base);

            if let Some(pid) = project_id {
                query = query.bind(pid);
            }
            if let Some(s) = search {
                query = query.bind(s);
            }
            if let Some(st) = status {
                query = query.bind(st);
            }
            if let Some(l) = label {
                let json = serde_json::json!({ "labels": [l] });
                query = query.bind(json);
            }

            let tasks = query.fetch_all(&mut **tx).await?;

            Ok(tasks)
        }
        .boxed()
    })
    .await?;

    Ok(tasks)
}