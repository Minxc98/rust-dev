mod config;
mod constant;
mod init;
mod route;
mod model;

use axum::{
    Router,
};


#[tokio::main]
async fn main() {
    init::initialize::init();
    // build our application with some routes
    let app = Router::new().nest("/api", route::api::api_router().await);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
