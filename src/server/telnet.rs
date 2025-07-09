use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::error::Error;
use std::collections::HashMap;
use crate::game::player::Player;

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

async fn prompt(writer: &mut tokio::net::tcp::OwnedWriteHalf, lines: &mut tokio::io::Lines<BufReader<tokio::net::tcp::OwnedReadHalf>>, message: &str) -> Result<String, Box<dyn Error>> {
    writer.write_all(message.as_bytes()).await?;
    writer.flush().await?;
    Ok(lines.next_line().await?.unwrap_or_default())
}


async fn handle_client(socket: tokio::net::TcpStream) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    writer.write_all(b"Welcome to your MUD engine!\r\n").await?;

    // -- Login / Character Creation --
    let name = prompt(&mut writer, &mut lines, "Enter your name: ").await?;
    let path = format!("assets/players/{}.json", name.to_lowercase());

    let player = if std::path::Path::new(&path).exists() {
        writer.write_all(b"Welcome back!\r\n").await?;
        Player::load_from_file(&name)?
    } else {
        writer.write_all(b"New character detected. Let's create one!\r\n").await?;

        let race = prompt(&mut writer, &mut lines, "Choose a race (human, elf, dwarf): ").await?;
        let class = prompt(&mut writer, &mut lines, "Choose a class (fighter, mage, rogue): ").await?;

        let new_player = Player {
            name: name.clone(),
            race,
            class,
            location: "start".to_string(),
            inventory: vec![],
            equipped: HashMap::new(),
        };

        new_player.save_to_file()?; // Save immediately
        writer.write_all(b"Character created! Entering the world...\r\n").await?;
        new_player
    };

    // -- Game Context and Command Loop --
    let mut context = GameContext::with_player(player);
    let processor = CommandProcessor::default();

    writer.write_all(b"\r\n> ").await?;

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

    // -- Save on exit --
    context.player.save_to_file()?;

    Ok(())
}
