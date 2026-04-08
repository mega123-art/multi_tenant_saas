use axum::{
    extract::{Extension, Query, State},
    Json,
};
use serde::Deserialize;
use validator::Validate;
use uuid::Uuid;
use sqlx::PgPool;
use serde_json::Value;

use crate::{
    db::tasks::{create_task, list_tasks},
    errors::ApiError,
    middleware::tenant::TenantContext,
};


//REQUEST


#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskRequest {
    pub project_id: Uuid,

    pub parent_task_id: Option<Uuid>,

    #[validate(length(min = 1))]
    pub title: String,

    pub description: Option<String>,

    pub metadata: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct TaskQuery {
    pub project_id: Option<Uuid>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub label: Option<String>,
}


//HANDLERS 


pub async fn create_task_handler(
    State((pool, _redis)): State<(PgPool, redis::Client)>,
    Extension(ctx): Extension<TenantContext>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<crate::models::Task>, ApiError> {
    payload.validate().map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let task = create_task(
        &pool,
        ctx.tenant_id,
        payload.project_id,
        payload.parent_task_id,
        payload.title,
        payload.description,
        payload.metadata.unwrap_or_else(|| serde_json::json!({})),
    )
    .await?;

    Ok(Json(task))
}

pub async fn list_tasks_handler(
    State((pool, _redis)): State<(PgPool, redis::Client)>,
    Extension(ctx): Extension<TenantContext>,
    Query(query): Query<TaskQuery>,
) -> Result<Json<Vec<crate::models::Task>>, ApiError> {
    let tasks = list_tasks(
        &pool,
        ctx.tenant_id,
        query.project_id,
        query.search,
        query.status,
        query.label,
    )
    .await?;

    Ok(Json(tasks))
}