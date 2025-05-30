use crate::controller::user_controller::{create_user, find_user_by_id, login_user, page_user, verify_user};
use crate::error::AppError;
use crate::init::app_state::AppState;
use axum::routing::{any, get, post};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;
use std::sync::Arc;
use crate::websocket::{broadcast_message, ws_handler, WsManager};

pub async fn api_router() -> Result<Router, AppError> {
    let state = AppState::new().await?;
    let ws_manager = Arc::new(WsManager::new());
    let user_router = Router::new()
        .route("/user/{id}", get(find_user_by_id))
        .route("/user", post(create_user))
        .route("/user/login", post(login_user))
        .route("/user/verify", post(verify_user))
        .route("/user/page", get(page_user))
        .with_state(state);

    let websocket_router = Router::new()
        .route("/ws", get(ws_handler))
        .route("/broadcast", post(broadcast_message))
        .with_state(ws_manager);
    let app_router = user_router.merge(websocket_router);
    Ok(set_router_layers(app_router))
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
