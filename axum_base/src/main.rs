mod config;
mod constant;
mod init;
mod route;
mod model;
mod error;
mod controller;

use anyhow::Result;

use tracing::{info};

#[tokio::main]
async fn main() -> Result<()>{
    init::initialize::init();

    // build our application with some routes
    let app = route::api::api_router().await?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service()).await?;
    
    Ok(())
}
