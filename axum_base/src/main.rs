mod config;
mod constant;
mod init;
mod route;
mod model;
mod error;

use axum::{
    Router,
};

use anyhow::Result;

use tracing::{info, level_filters::LevelFilter as Level};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()>{
    // init::initialize::init();

    let layer = Layer::new().with_filter(Level::INFO);
    tracing_subscriber::registry().with(layer).init();
    // build our application with some routes
    let app = route::api::api_router().await?;

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service()).await?;
    
    Ok(())
}
