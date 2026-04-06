use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};

use crate::errors::ApiError;


//GET FROM CACHE 

pub async fn get_cached<T: DeserializeOwned>(
    client: &redis::Client,
    key: &str,
) -> Result<Option<T>, ApiError> {
    let mut conn = client.get_multiplexed_async_connection().await?;

    let data: Option<String> = conn.get(key).await?;

    match data {
        Some(json) => {
            let value = serde_json::from_str(&json)
                .map_err(|_| ApiError::InternalServerError)?;
            Ok(Some(value))
        }
        None => Ok(None),
    }
}


//SET CACHE


pub async fn set_cache<T: Serialize>(
    client: &redis::Client,
    key: &str,
    value: &T,
    ttl_secs: u64,
) -> Result<(), ApiError> {
    let mut conn = client.get_multiplexed_async_connection().await?;

    let json = serde_json::to_string(value)
        .map_err(|_| ApiError::InternalServerError)?;

    let _: () = conn.set_ex(key, json, ttl_secs).await?;

    Ok(())
}


//DELETE CACHE


pub async fn delete_cache(
    client: &redis::Client,
    key: &str,
) -> Result<(), ApiError> {
    let mut conn = client.get_multiplexed_async_connection().await?;

    let _: () = conn.del(key).await?;

    Ok(())
}