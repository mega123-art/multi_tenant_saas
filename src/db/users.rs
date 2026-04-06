use sqlx::PgPool;
use uuid::Uuid;

use crate::models::User;
use crate::errors::ApiError;
use crate::db::utils::with_tenant;


// CREATE USER 


pub async fn create_user(
    pool: &PgPool,
    tenant_id: Uuid,
    name: String,
    email: String,
) -> Result<User, ApiError> {
    let user = with_tenant(pool, tenant_id, |tx| async move {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (tenant_id, name, email)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, email, role, created_at
            "#,
            tenant_id,
            name,
            email
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(user)
    })
    .await?;

    Ok(user)
}


//LIST USERS 


pub async fn list_users(
    pool: &PgPool,
    tenant_id: Uuid,
) -> Result<Vec<User>, ApiError> {
    let users = with_tenant(pool, tenant_id, |tx| async move {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, tenant_id, name, email, role, created_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&mut **tx)
        .await?;

        Ok(users)
    })
    .await?;

    Ok(users)
}

//
// ===== GET USER BY ID =====
//

pub async fn get_user(
    pool: &PgPool,
    tenant_id: Uuid,
    user_id: Uuid,
) -> Result<User, ApiError> {
    let user = with_tenant(pool, tenant_id, |tx| async move {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, tenant_id, name, email, role, created_at
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&mut **tx)
        .await?;

        match user {
            Some(u) => Ok(u),
            None => Err(ApiError::NotFound),
        }
    })
    .await?;

    Ok(user)
}