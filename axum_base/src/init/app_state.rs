use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use crate::error::AppError;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

pub(crate) struct AppStateInner {
    pub(crate) pool: sqlx::PgPool,
    pub(crate) redis_client: redis::Client,
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
        let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost/chat")
            .await
            .map_err(|_| {
                AppError::Database(sqlx::Error::Configuration(
                    "Failed to connect to database".into(),
                ))
            })?;
        let redis_client = redis::Client::open("redis://127.0.0.1/")?;

        Ok(Self {
            inner: Arc::new(AppStateInner {
                pool,
                redis_client
            }),
        })
    }
}