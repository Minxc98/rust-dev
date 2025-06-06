use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use crate::error::AppError;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

pub(crate) struct AppStateInner {
    pub(crate) pool: sqlx::PgPool,
    pub(crate) _redis_client: redis::Client,
    pub(crate) pem:String,
    pub(crate) _kafka_url: String,
}


impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("pool", &self.pool)
            .finish()
    }
}

impl AppState {
    pub async fn new() -> Result<Self, AppError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let kafka_url = env::var("KAFKA_URL").expect("KAFKA_URL must be set");
        let pem = env::var("PEM").expect("PEM must be set");
        let pool = sqlx::PgPool::connect(&database_url)
            .await
            .map_err(|_| {
                AppError::Database(sqlx::Error::Configuration(
                    "Failed to connect to database".into(),
                ))
            })?;
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        let redis_client = redis::Client::open(redis_url)?;

        Ok(Self {
            inner: Arc::new(AppStateInner {
                pool,
                _redis_client: redis_client,
                pem,
                _kafka_url: kafka_url
            }),
        })
    }
}