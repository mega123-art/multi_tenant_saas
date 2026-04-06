use axum::{
    Json,
    extract::{Extension, Path, State},
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::projects::{create_project, delete_project, get_project, list_projects},
    errors::ApiError,
    middleware::tenant::TenantContext,
    cache::{get_cached, set_cache, delete_cache},
};
use redis::Client as RedisClient;
//REQUEST

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(min = 1))]
    pub name: String,

    pub description: Option<String>,
}

//HANDLERS

pub async fn create_project_handler(
    State((pool, redis)): State<(PgPool, RedisClient)>,
    Extension(ctx): Extension<TenantContext>,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<crate::models::Project>, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let project = create_project(&pool, ctx.tenant_id, payload.name, payload.description).await?;

    // cache invalidation
    let key = format!("project_list:{}", ctx.tenant_id);
    delete_cache(&redis, &key).await?;

    Ok(Json(project))
}

pub async fn list_projects_handler(
    State((pool, redis)): State<(PgPool, RedisClient)>,
    Extension(ctx): Extension<TenantContext>,
) -> Result<Json<Vec<crate::models::Project>>, ApiError> {
    let cache_key = format!("project_list:{}", ctx.tenant_id);

    //Try cache
    if let Some(cached) = get_cached::<Vec<crate::models::Project>>(&redis, &cache_key).await? {
        println!("CACHE HIT");
        return Ok(Json(cached));
    }

    //DB fallback
    println!("CACHE MISS");

    let projects = list_projects(&pool, ctx.tenant_id).await?;

    //Store in cache (TTL = 300s)
    set_cache(&redis, &cache_key, &projects, 300).await?;

    Ok(Json(projects))
}

pub async fn get_project_handler(
    State((pool, _)): State<(PgPool, RedisClient)>,
    Extension(ctx): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::models::Project>, ApiError> {
    let project = get_project(&pool, ctx.tenant_id, id).await?;

    Ok(Json(project))
}

pub async fn delete_project_handler(
    State((pool, redis)): State<(PgPool, RedisClient)>,
    Extension(ctx): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<(), ApiError> {
    delete_project(&pool, ctx.tenant_id, id).await?;

    // cache invalidation
    let key = format!("project_list:{}", ctx.tenant_id);
    delete_cache(&redis, &key).await?;

    Ok(())
}
