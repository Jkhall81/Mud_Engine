use mud_engine::db;
use mud_engine::server::telnet;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Loads .env file into env vars

    let pool = match db::establish_db_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e);
            return;
        }
    };

    if let Err(e) = telnet::start_telnet_server(pool).await {
        eprintln!("Server error: {:?}", e);
    }
}
