use crate::controller::user_controller::{create_user, find_user_by_id, login_user, page_user, verify_user,handler};
use crate::error::AppError;
use crate::init::app_state::AppState;
use axum::routing::{any, get, post};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;

pub async fn api_router() -> Result<Router, AppError> {
    let state = AppState::new().await?;
    let router = Router::new()
        .route("/user/{id}", get(find_user_by_id))
        .route("/user", post(create_user))
        .route("/user/login", post(login_user))
        .route("/user/verify", post(verify_user))
        .route("/user/page", get(page_user))
        .route("/user/test/ws", any(handler))
        .with_state(state);
    Ok(set_router_layers(router))
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
