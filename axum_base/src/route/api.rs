use crate::config;
use crate::constant::type_config::{internal_error, PostgresConnectionPool, RedisConnectionPool};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use bb8_redis::redis::AsyncCommands;

pub async fn api_router() -> Router {
    Router::new()
        .route("/redis",get(using_connection_pool_extractor))
        .with_state(config::redis_pool::init_pool().await)
        .merge(
           Router::new()
              .route("/postgres",get(select_test_pg))
               .with_state(config::postgres_pool::init_pool().await)
        )
}
async fn using_connection_pool_extractor(
    State(pool): State<RedisConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    let result: String = conn.get("foo").await.map_err(internal_error)?;
    Ok(result)
}


async fn select_test_pg(
    State(pool): State<PostgresConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internal_error)?;
    let two: i32 = row.try_get(0).map_err(internal_error)?;

    Ok(two.to_string())
}