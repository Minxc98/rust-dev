cd ..
sqlx database create
sqlx migrate run
cargo sqlx prepare --workspace