use crate::config;
use crate::error::AppError;
use crate::init::app_state::AppState;
use crate::model::user;
use crate::model::user::SignInUser;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{AppendHeaders, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router};
use sqlx::{Error, PgPool, Pool, Postgres};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub async fn api_router() -> Result<Router, AppError> {
    use config::*;
    let state = AppState::new().await?;
    let router = Router::new()
        .route("/pg/{id}",get(update_user))
        .with_state(state)
        ;
    Ok(set_router_layers(router))
}

#[derive(serde::Deserialize)]
struct PathParams {
    id: i32,
}
async fn update_user(
    State(context): State<AppState>,
    Path(params): Path<PathParams>,
) -> Result<impl IntoResponse, AppError> {

    let pg_pool = &context.pool;
    let result = select_user(pg_pool, params.id).await?;
    Ok(Json(result))
}

async fn select_user(pool: &PgPool,id: i32)-> Result<SignInUser,AppError>{

    let user = sqlx::query_as!(
        SignInUser,
        r"
        select username, email
        from users
        where id = $1
       ",
        id
    )
        .fetch_one(pool)
        .await?;
    Ok(user)
    
}



pub fn set_router_layers(app: Router) -> Router {
    app.layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Micros),
                    ),
            )
      
    )
}
