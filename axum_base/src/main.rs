mod config;
mod constant;
mod init;
mod route;
mod model;
mod error;
mod controller;
mod kafka;

use anyhow::Result;

use tracing::{info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::init::app_state::AppState;
use crate::controller::user_controller::ApiDoc;

#[tokio::main]
async fn main() -> Result<()>{
    init::initialize::init();

    let api_doc = ApiDoc::openapi();

    let app = route::api::api_router().await?
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", api_doc));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("listening on {}", listener.local_addr()?);
    info!("swagger-ui: http://127.0.0.1:3000/swagger-ui");

    axum::serve(listener, app.into_make_service()).await?;
    
    Ok(())
}
