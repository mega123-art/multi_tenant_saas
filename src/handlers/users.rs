use axum::{
    extract::{Extension, Path, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::users::{create_user, list_users, get_user},
    errors::ApiError,
    middleware::tenant::TenantContext,
};
use sqlx::PgPool;


//REQUEST STRUCTS


#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(email)]
    pub email: String,
}


//HANDLERS


pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Extension(ctx): Extension<TenantContext>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<crate::models::User>, ApiError> {
    payload.validate().map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let user = create_user(
        &pool,
        ctx.tenant_id,
        payload.name,
        payload.email,
    )
    .await?;

    Ok(Json(user))
}

pub async fn list_users_handler(
    State(pool): State<PgPool>,
    Extension(ctx): Extension<TenantContext>,
) -> Result<Json<Vec<crate::models::User>>, ApiError> {
    let users = list_users(&pool, ctx.tenant_id).await?;

    Ok(Json(users))
}

pub async fn get_user_handler(
    State(pool): State<PgPool>,
    Extension(ctx): Extension<TenantContext>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<crate::models::User>, ApiError> {
    let user = get_user(&pool, ctx.tenant_id, user_id).await?;

    Ok(Json(user))
}