use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn establish_db_pool() -> Result<PgPool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env or environment");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}