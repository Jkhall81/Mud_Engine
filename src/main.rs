use mud_engine::server::telnet;

#[tokio::main]
async fn main() {
    if let Err(e) = telnet::start_telnet_server().await {
        eprintln!("Server error: {:?}", e);
    }
}
