
use axum::{
    extract::{State, Path},
    response::IntoResponse,
    Json,
};
use crate::error::AppError;
use crate::init::app_state::AppState;

pub(crate) async fn update_user(
    State(context): State<AppState>,
    Path(params): Path<i32>,
) -> Result<impl IntoResponse, AppError> {

    let pg_pool = &context.pool;
    let result = crate::model::user::SignInUser::select_user(pg_pool, params).await?;
    Ok(Json(result))
}

