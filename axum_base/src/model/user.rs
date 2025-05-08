use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub workspace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInUser {
    pub username: String,
    pub email: String,
}

impl SignInUser {
    pub(crate) async fn select_user(pool: &PgPool, id: i32) -> Result<SignInUser, AppError> {
        let user = sqlx::query_as!(
            SignInUser,
            r"
        select username, email
        from users
        where id = $1
       ",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(user)
    }
}
