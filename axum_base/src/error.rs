use axum::{
    http::{StatusCode},
    response::{IntoResponse, Response},
};
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("Invalid credentials")]
    InvalidCredentials,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::Redis(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::Json(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            Self::Jwt(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::Validation(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            Self::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        }
    }
}
