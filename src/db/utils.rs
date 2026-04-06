use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::errors::ApiError;

pub async fn with_tenant<F, Fut, T>(
    pool: &PgPool,
    tenant_id: Uuid,
    f: F,
) -> Result<T, ApiError>
where
    F: FnOnce(&mut Transaction<'_, Postgres>) -> Fut,
    Fut: std::future::Future<Output = Result<T, ApiError>>,
{
    //start transaction
    let mut tx = pool.begin().await?;

    //set tenant (scoped to transaction)
    sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(tenant_id.to_string())
        .execute(&mut *tx)
        .await?;

    //run user logic
    let result = f(&mut tx).await?;

    //commit
    tx.commit().await?;

    Ok(result)
}

// we dont use set because we want to ensure that tenant_id is always set for any query run within with_tenant, and using set would allow users to accidentally override it. By using set local, we ensure that the tenant_id is only set for the duration of the transaction and cannot be overridden by user code.