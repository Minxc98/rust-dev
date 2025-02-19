use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub async fn init_pool() -> Pool<PostgresConnectionManager<NoTls>> {
    let manager = PostgresConnectionManager::new_from_stringlike("host=localhost user=postgres password=postgres", NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    pool
}
