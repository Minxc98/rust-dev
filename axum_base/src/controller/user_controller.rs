use axum::{
    extract::{State, Path},
    response::IntoResponse,
    Json,
};
use crate::error::AppError;
use crate::init::app_state::AppState;
use sha2::{Sha256, Digest};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa::ToSchema;
use crate::model::user::{BaseUserInfo, CreateUser, LoginUser};
use validator::Validate;

#[derive(OpenApi)]
#[openapi(
    paths(
        find_user_by_id,
        create_user,
        login_user
    ),
    components(
        schemas(crate::model::user::CreateUser, crate::model::user::BaseUserInfo, crate::model::user::LoginUser)
    ),
    tags(
        (name = "users", description = "User management endpoints.")
    )
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/user/{id}",
    responses(
        (status = 200, description = "User found", body = crate::model::user::BaseUserInfo),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = i32, Path, description = "User ID")
    )
)]
pub(crate) async fn find_user_by_id(
    State(context): State<AppState>,
    Path(params): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let pg_pool = &context.pool;
    let result = crate::model::user::BaseUserInfo::select_user(pg_pool, params).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/user",
    request_body = crate::model::user::CreateUser,
    responses(
        (status = 201, description = "User created", body = String),
        (status = 400, description = "Invalid input")
    )
)]
pub(crate) async fn create_user(
    State(context): State<AppState>,
    Json(user): Json<crate::model::user::CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    user.validate()?;
    let pg_pool = &context.pool;
    let token = crate::model::user::CreateUser::insert_user(pg_pool, &user).await?;
    Ok(Json(token))
}

#[utoipa::path(
    post,
    path = "/user/login",
    request_body = crate::model::user::LoginUser,
    responses(
        (status = 200, description = "Login successful", body = String),
        (status = 401, description = "Invalid credentials")
    )
)]
pub(crate) async fn login_user(
    State(context): State<AppState>,
    Json(user): Json<crate::model::user::LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    let pg_pool = &context.pool;
    let token = crate::model::user::LoginUser::verify_user(pg_pool, &user).await?;
    Ok(Json(token))
}

