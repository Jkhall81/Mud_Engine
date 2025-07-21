use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use mud_engine::loader::load_rooms_from_dir;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("DB connection failed");

    load_rooms_from_dir(&pool, "assets/rooms/30")
        .await
        .expect("Room loading failed");

    println!("Rooms loaded successfully.");
}
