use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sha2::{Sha256, Digest};
use jsonwebtoken::{encode, EncodingKey, Header, TokenData, decode, DecodingKey, Validation};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BaseUserInfo {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VerifyUserInfo {
    pub username: String,
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Claims {
    sub: String,
    exp: usize,
}

impl BaseUserInfo {
    pub(crate) async fn select_user(pool: &PgPool, id: i32) -> Result<BaseUserInfo, AppError> {
        let user = sqlx::query_as!(
            BaseUserInfo,
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

impl VerifyUserInfo {
    pub(crate) async fn verify_user(pg_pool: PgPool, user: &VerifyUserInfo) -> Result<bool, AppError> {
        let mut hasher = Sha256::new();
        hasher.update(user.password.as_bytes());
        let password_hash = format!("{:x}", hasher.finalize());
        let user = sqlx::query!(
            r"
            SELECT id
            FROM users
            WHERE username = $1 AND password_hash = $2
            ",
            user.username,
            password_hash
        )
            .fetch_optional(&pg_pool)
            .await?
            .is_some();
        Ok(user)
    }
}

impl CreateUser {
    pub(crate) async fn insert_user(pool: &PgPool, user: &CreateUser) -> Result<String, AppError> {
        let mut hasher = Sha256::new();
        hasher.update(user.password.as_bytes());
        let password_hash = format!("{:x}", hasher.finalize());
    
        let user_id = sqlx::query!(
            r"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id
            ",
            user.username,
            user.email,
            password_hash
        )
        .fetch_one(pool)
        .await?
        .id;
    
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };
    
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(b"your_secret_key"),
        )?;
    
        Ok(token)
    }

    pub(crate) fn validate_token(token: &str) -> Result<Claims, AppError> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(b"your_secret_key"),
            &validation,
        )?;
        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_insert_user() {
        let pool = PgPool::connect("postgres://postgres:postgres@localhost/dev").await.unwrap();
        let user = CreateUser {
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let token = CreateUser::insert_user(&pool, &user).await.unwrap();
        let claims = CreateUser::validate_token(&token).unwrap();
        //not null
        assert!(!claims.sub.is_empty());
        assert!(claims.exp > 0);
    }
}
