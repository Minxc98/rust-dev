use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sha2::{Sha256, Digest};
use jsonwebtoken::{encode, EncodingKey, Header, TokenData, decode, DecodingKey, Validation};
use sqlx_paginated::{paginated_query_as, PaginatedResponse, QueryParamsBuilder, QuerySortDirection};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

fn validate_password_complexity(password: &str) -> Result<(), ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| "@$!%*?&".contains(c));

    if !has_uppercase || !has_lowercase || !has_digit || !has_special {
        return Err(ValidationError::new("password must contain at least one uppercase letter, one lowercase letter, one number and one special character"));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 20, message = "username length must be between 3 and 20"))]
    pub username: String,
    #[validate(email(message = "invalid email format"))]
    pub email: String,
    #[validate(
        length(min = 6, max = 32, message = "password length must be between 6 and 32"),
        custom = "validate_password_complexity"
    )]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate,sqlx::FromRow,Default)]
pub struct BaseUserInfo {
    #[validate(length(min = 3, max = 20, message = "username length must be between 3 and 20"))]
    pub username: String,
    #[validate(email(message = "invalid email format"))]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct LoginUser {
    #[validate(length(min = 3, max = 20, message = "username length must be between 3 and 20"))]
    pub username: String,
    #[validate(length(min = 6, max = 32, message = "password length must be between 6 and 32"))]
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Claims {
    sub: String,
    exp: usize,
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

    pub(crate) async fn page_user(pool: &PgPool, page: i32, page_size: i32) -> Result<PaginatedResponse<BaseUserInfo>, AppError> {
        let params = QueryParamsBuilder::<BaseUserInfo>::new()
        .with_pagination(page as i64, page_size as i64)
        .build();
    let paginated_response = paginated_query_as!(BaseUserInfo, "SELECT * FROM users")
        // Alternative function call example (if macros don't fit your use case):
        // paginated_query_as::<User>("SELECT * FROM users")
        .with_params(params)
        .fetch_paginated(pool)
        .await?;

    Ok(paginated_response)
    }
}

impl LoginUser {
    pub(crate) async fn verify_user(pg_pool: &PgPool, pem: &str, user: &LoginUser) -> Result<String, AppError> {
        let mut hasher = Sha256::new();
        hasher.update(user.password.as_bytes());
        let password_hash = format!("{:x}", hasher.finalize());
        let user_id = sqlx::query!(
            r"
            SELECT id
            FROM users
            WHERE username = $1 AND password_hash = $2
            ",
            user.username,
            password_hash
        )
            .fetch_optional(pg_pool)
            .await?
            .ok_or(AppError::InvalidCredentials)?
            .id;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(pem.as_bytes()),
        )?;
        Ok(token)
    }
}

impl CreateUser {
    pub(crate) async fn insert_user(pool: &PgPool,pem: &str, user: &CreateUser) -> Result<String, AppError> {
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
            &EncodingKey::from_secret(pem.as_bytes()),
        )?;
    
        Ok(token)
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::test_utils::TestDatabase;

    #[tokio::test]
    async fn test_insert_user() {
        let test_db = TestDatabase::new().await;
        let pool = test_db.pool;

        let user = CreateUser {
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password: "password123@".to_string(),
        };
        let pem = "your_secret_key";
        let token = CreateUser::insert_user(&pool, &pem, &user).await.unwrap();
        let claims: Claims = decode(
            &token,
            &DecodingKey::from_secret(pem.as_bytes()),
            &Validation::default()
        ).unwrap().claims;
        //not null
        assert!(!claims.sub.is_empty());
        assert!(claims.exp > 0);
    }

    #[tokio::test]
    async fn test_page_user() {
        let test_db = TestDatabase::new().await;
        let pool = test_db.pool;

        // 清理数据库
        sqlx::query!("DELETE FROM users WHERE id > 0").execute(&pool).await.unwrap();

        // 插入测试数据
        let users = vec![
            CreateUser {
                username: "user1".to_string(),
                email: "user1@example.com".to_string(),
                password: "Password123@".to_string(),
            },
            CreateUser {
                username: "user2".to_string(),
                email: "user2@example.com".to_string(),
                password: "Password123@".to_string(),
            },
            CreateUser {
                username: "user3".to_string(),
                email: "user3@example.com".to_string(),
                password: "Password123@".to_string(),
            },
        ];

        let pem = "your_secret_key";
        for user in users {
            CreateUser::insert_user(&pool, &pem, &user).await.unwrap();
        }

        // 测试第一页，每页2条数据
        let page1 = BaseUserInfo::page_user(&pool, 1, 2).await.unwrap();
        assert_eq!(page1.records.len(), 2);
        assert_eq!(page1.total, Some(3));
        let pagination1 = page1.pagination.unwrap();
        assert_eq!(pagination1.page, 1);
        assert_eq!(pagination1.page_size, 2);

        // 测试第二页，每页2条数据
        let page2 = BaseUserInfo::page_user(&pool, 2, 2).await.unwrap();
        assert_eq!(page2.records.len(), 1);
        assert_eq!(page2.total, Some(3));
        let pagination2 = page2.pagination.unwrap();
        assert_eq!(pagination2.page, 2);
        assert_eq!(pagination2.page_size, 2);

        // 测试超出范围的页码
        let page3 = BaseUserInfo::page_user(&pool, 3, 2).await.unwrap();
        assert_eq!(page3.records.len(), 0);
        assert_eq!(page3.total, Some(3));
        let pagination3 = page3.pagination.unwrap();
        assert_eq!(pagination3.page, 3);
        assert_eq!(pagination3.page_size, 2);
    }
}
