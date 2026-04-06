use sqlx::{PgPool, PgConnection};
use uuid::Uuid;

use crate::models::Tenant;
use crate::errors::ApiError;


//CREATE TENANT 


pub async fn create_tenant(
    pool: &PgPool,
    name: String,
    slug: String,
) -> Result<Tenant, ApiError> {
    let tenant = sqlx::query_as!(
        Tenant,
        r#"
        INSERT INTO tenants (name, slug)
        VALUES ($1, $2)
        RETURNING id, name, slug, created_at
        "#,
        name,
        slug
    )
    .fetch_one(pool)
    .await?;

    Ok(tenant)
}


// GET TENANT BY SLUG


pub async fn get_tenant_by_slug(
    pool: &PgPool,
    slug: &str,
) -> Result<Tenant, ApiError> {
    let tenant = sqlx::query_as!(
        Tenant,
        r#"
        SELECT id, name, slug, created_at
        FROM tenants
        WHERE slug = $1
        "#,
        slug
    )
    .fetch_optional(pool)
    .await?;

    match tenant {
        Some(t) => Ok(t),
        None => Err(ApiError::NotFound),
    }
}