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
) -> Result<Vec<Task>, ApiError> {
    let tasks = with_tenant(pool, tenant_id, |tx| async move {
        let tasks = if let Some(pid) = project_id {
            sqlx::query_as!(
                Task,
                r#"
                SELECT
                    id, tenant_id, project_id, parent_task_id,
                    title, description, status, priority,
                    metadata, version, created_at, updated_at
                FROM tasks
                WHERE project_id = $1
                ORDER BY created_at DESC
                "#,
                pid
            )
            .fetch_all(&mut **tx)
            .await?
        } else {
            sqlx::query_as!(
                Task,
                r#"
                SELECT
                    id, tenant_id, project_id, parent_task_id,
                    title, description, status, priority,
                    metadata, version, created_at, updated_at
                FROM tasks
                ORDER BY created_at DESC
                "#
            )
            .fetch_all(&mut **tx)
            .await?
        };

        Ok(tasks)
    }.boxed())
    .await?;

    Ok(tasks)
}