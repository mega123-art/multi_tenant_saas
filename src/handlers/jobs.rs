use axum::{
    Json,
    extract::{Extension, State},
};
use serde::Deserialize;
use serde_json::Value;
use sqlx::PgPool;

use crate::{
    db::jobs::{create_job, list_pending_jobs},
    errors::ApiError,
    middleware::tenant::TenantContext,
};

#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub job_type: String,
    pub payload: Value,
}

pub async fn create_job_handler(
    State((pool, _)): State<(PgPool, redis::Client)>,
    Extension(ctx): Extension<TenantContext>,
    Json(payload): Json<CreateJobRequest>,
) -> Result<Json<crate::models::Job>, ApiError> {
    let job = create_job(&pool, ctx.tenant_id, payload.job_type, payload.payload).await?;

    Ok(Json(job))
}

pub async fn list_jobs_handler(
    State((pool, _)): State<(PgPool, redis::Client)>,
    Extension(ctx): Extension<TenantContext>,
) -> Result<Json<Vec<crate::models::Job>>, ApiError> {
    let jobs = list_pending_jobs(&pool, ctx.tenant_id).await?;

    Ok(Json(jobs))
}
