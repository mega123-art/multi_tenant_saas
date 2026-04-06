use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::tenants::get_tenant_by_slug;
use crate::errors::ApiError;

#[derive(Clone, Copy)]
pub struct TenantContext {
    pub tenant_id: Uuid,
}

pub async fn tenant_middleware(
    State((pool, _)): State<(PgPool, redis::Client)>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    // Read header
    let headers: &HeaderMap = req.headers();

    let slug = headers
        .get("X-Tenant-Slug")
        .and_then(|v| v.to_str().ok())
        .ok_or(ApiError::BadRequest("Missing X-Tenant-Slug".into()))?;

    //Fetch tenant
    let tenant = get_tenant_by_slug(&pool, slug).await?;

    //Attach tenant_id to request extensions
    req.extensions_mut().insert(TenantContext {
        tenant_id: tenant.id,
    });

    //Continue request
    let response = next.run(req).await;

    Ok(response)
}
