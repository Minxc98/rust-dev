use crate::config;
use crate::constant::type_config::{internal_error, PostgresConnectionPool, RedisConnectionPool};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;
use bb8_redis::redis::AsyncCommands;
use sqlx::{Pool, Postgres};
use crate::model::user;
use crate::model::user::SignInUser;

pub async fn api_router() -> Router {
    use config::*;
    Router::new()
        .route("/redis",get(using_connection_pool_extractor))
        .with_state(redis_pool::init_pool().await)
        .merge(
           Router::new()
              .route("/postgres",get(select_test_pg))
               .with_state(postgres_pool::init_sqlx_pool().await)
        )
        .merge(
            Router::new()
                .route("/insert/{id}", get(update_user)) // 注意这里的路径参数 :id
                .with_state(postgres_pool::init_sqlx_pool().await)
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
    State(pool): State<Pool<Postgres>>,
) -> Result<String, (StatusCode, String)> {
    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let res :(i64,)= sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await.map_err(internal_error)?;
    assert_eq!(res.0,150_i64);

    Ok(1.to_string())
}
#[derive(serde::Deserialize)]
struct PathParams {
    id: i32,
}
async fn update_user(
    State(pool): State<Pool<Postgres>>,
    Path(params): Path<PathParams>,
) -> Result<String, (StatusCode, String)> {
    use crate::model;
    let user = sqlx::query_as!(SignInUser,
       r"
        select username, email
        from users
        where id = $1
       ", params.id)
        .fetch_one(&pool).await.map_err(internal_error).unwrap();
    Ok(user.email)
}
