use axum::{
    extract::{State, Path},
    response::IntoResponse,
    Json,
};
use crate::error::AppError;
use crate::init::app_state::AppState;
use utoipa::OpenApi;
use utoipa::ToSchema;
use crate::model::user::{BaseUserInfo, CreateUser, LoginUser};
use validator::Validate;
use serde::{Deserialize, Serialize};


#[derive(OpenApi)]
#[openapi(
    paths(
        find_user_by_id,
        create_user,
        login_user,
        verify_user,
        page_user
    ),
    components(
        schemas(
            crate::model::user::CreateUser, 
            crate::model::user::BaseUserInfo, 
            crate::model::user::LoginUser,
            crate::model::user::LoginUser,
            crate::controller::user_controller::PageUserQuery
        )
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
        (status = 200, description = "User found", body = BaseUserInfo),
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
    let result = BaseUserInfo::select_user(pg_pool, params).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/user",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created", body = String),
        (status = 400, description = "Invalid input")
    )
)]
pub(crate) async fn create_user(
    State(context): State<AppState>,
    Json(user): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    user.validate()?;
    let pg_pool = &context.pool;
    let pem = &context.pem;
    let token = CreateUser::create_user(pg_pool,pem, &user).await?;
    Ok(Json(token))
}

#[utoipa::path(
    post,
    path = "/user/login",
    request_body = LoginUser,
    responses(
        (status = 200, description = "Login successful", body = String),
        (status = 401, description = "Invalid credentials")
    )
)]
pub(crate) async fn login_user(
    State(context): State<AppState>,
    Json(user): Json<LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    let pg_pool = &context.pool;
    let pem = &context.pem;
    let token = LoginUser::verify_user(pg_pool, pem, &user).await?;
    Ok(Json(token))
}

#[utoipa::path(
    post,
    path = "/user/verify",
    request_body = LoginUser,
    responses(
        (status = 200, description = "Verify successful", body = String),
        (status = 401, description = "Invalid credentials")
    )
)]
pub(crate) async fn verify_user(
    State(context): State<AppState>,
    Json(user): Json<LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    let pg_pool = &context.pool;
    let pem = &context.pem;
    let token = LoginUser::verify_user(pg_pool, pem, &user).await?;
    Ok(Json(token))
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PageUserQuery {
    page: i32,
    page_size: i32,
}

#[utoipa::path(
    get,
    path = "/user/page",
    responses(
        (status = 200, description = "Page found", body = BaseUserInfo)
    )
)]
pub(crate) async fn page_user(
    State(context): State<AppState>,
    Json(query): Json<PageUserQuery>,
) -> Result<impl IntoResponse, AppError> {
    let pg_pool = &context.pool;
    let result = BaseUserInfo::page_user(pg_pool, query.page, query.page_size).await?;
    Ok(Json(result))
}


