use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use tokio_postgres::NoTls;

pub async fn init_pool() -> Pool<PostgresConnectionManager<NoTls>> {
    let manager = PostgresConnectionManager::new_from_stringlike("host=localhost user=postgres password=postgres", NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    pool
}


pub async fn init_sqlx_pool() -> sqlx::Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/chat").await.unwrap()
}