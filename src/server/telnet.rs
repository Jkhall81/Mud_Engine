use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::error::Error;

use crate::game::{parser::CommandProcessor, state::GameContext};

pub async fn start_telnet_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:4000").await?;
    println!("Telnet server running on port 4000...");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error with client {}: {:?}", addr, e);
            }
        });
    }
}

async fn handle_client(socket: tokio::net::TcpStream) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    let mut context = GameContext::new("start".to_string()); // Load player at start room
    let processor = CommandProcessor::default();

    writer.write_all(b"Welcome to your MUD engine.\r\n> ").await?;

    while let Some(line) = lines.next_line().await? {
        let input = line.trim();

        if input.eq_ignore_ascii_case("quit") {
            writer.write_all(b"Goodbye!\r\n").await?;
            break;
        }

        let response = processor.handle(input, &mut context);
        writer.write_all(response.as_bytes()).await?;
        writer.write_all(b"\r\n> ").await?;
    }

    Ok(())
}
