use crate::error::AppError;
use crate::init::app_state::AppState;
use crate::model::user::SignInUser;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use crate::controller::user_controller::update_user;
pub async fn api_router() -> Result<Router, AppError> {
    
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
