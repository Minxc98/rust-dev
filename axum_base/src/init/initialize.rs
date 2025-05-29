use tracing::{level_filters::LevelFilter as Level};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer as _};
use testcontainers::{clients};
use std::sync::OnceLock;

pub fn init() {
    init_log();
}

pub fn init_log() {
    let layer = Layer::new().with_filter(Level::DEBUG);
    tracing_subscriber::registry().with(layer).init();
}

static _CLI: OnceLock<clients::Cli> = OnceLock::new();

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use std::process::Command;
    use std::sync::Once;
    use std::time::Duration;
    use sqlx::PgPool;
    use testcontainers::core::WaitFor;
    use testcontainers::{Container, GenericImage, RunnableImage};

    static _INIT: Once = Once::new();

    pub struct TestDatabase {
        pub _container: Container<'static, GenericImage>,
        pub pool: PgPool,
    }

    impl TestDatabase {
        pub async fn new() -> Self {
            let cli = _CLI.get_or_init(|| clients::Cli::default());
            let image = RunnableImage::from(
                GenericImage::new("postgres", "latest")
                    .with_env_var("POSTGRES_USER", "postgres")
                    .with_env_var("POSTGRES_PASSWORD", "postgres")
                    .with_env_var("POSTGRES_DB", "dev")
                    .with_exposed_port(5432)
                    .with_wait_for(WaitFor::message_on_stderr("database system is ready to accept connections"))
            );
            let container = cli.run(image);
            let port = container.get_host_port_ipv4(5432);

            // 等待数据库完全启动
            tokio::time::sleep(Duration::from_secs(2)).await;

            // 运行 SQL 命令创建数据库和表
            let db_url = format!("postgres://postgres:postgres@localhost:{}/dev", port);
            
            // 创建迁移文件目录（在项目根目录下）
            let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            let project_root = manifest_dir.parent().unwrap();
            let migrations_dir = project_root.join("migrations");
            std::fs::create_dir_all(&migrations_dir).expect("Failed to create migrations directory");

            let output = Command::new("sqlx")
                .args(&["database", "create"])
                .env("DATABASE_URL", &db_url)
                .current_dir(project_root)
                .output()
                .expect("Failed to execute command");

            if !output.status.success() {
                panic!("Failed to create database: {}", String::from_utf8_lossy(&output.stderr));
            }

            let output = Command::new("sqlx")
                .args(&["migrate", "run"])
                .env("DATABASE_URL", &db_url)
                .current_dir(project_root)
                .output()
                .expect("Failed to execute command");

            if !output.status.success() {
                panic!("Failed to run migrations: {}", String::from_utf8_lossy(&output.stderr));
            }

            let pool = PgPool::connect(&db_url).await.unwrap();

            Self {
                _container: container,
                pool,
            }
        }
    }
}
