use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    db::tenants::{create_tenant, get_tenant_by_slug},
    errors::ApiError,
};

//REQUEST

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTenantRequest {
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1))]
    pub slug: String,
}

//
// ===== HANDLERS =====
//

pub async fn create_tenant_handler(
    State((pool, _)): State<(PgPool, redis::Client)>,
    Json(payload): Json<CreateTenantRequest>,
) -> Result<Json<crate::models::Tenant>, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let tenant = create_tenant(&pool, payload.name, payload.slug).await?;

    Ok(Json(tenant))
}

pub async fn get_tenant_handler(
    State((pool, _)): State<(PgPool, redis::Client)>,
    Path(slug): Path<String>,
) -> Result<Json<crate::models::Tenant>, ApiError> {
    let tenant = get_tenant_by_slug(&pool, &slug).await?;

    Ok(Json(tenant))
}
