use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::error::Error;
use std::collections::HashMap;
use sqlx::PgPool;

use crate::game::player::Player;
use crate::game::{parser::CommandProcessor, state::GameContext};
use crate::game::character_templates::{get_race_mods, get_class_mods, StatBlock};

pub async fn start_telnet_server(pool: PgPool) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:4000").await?;
    println!("Telnet server running on port 4000...");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        let pool = pool.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, pool).await {
                eprintln!("Error with client {}: {:?}", addr, e);
            }
        });
    }
}

async fn prompt(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    lines: &mut tokio::io::Lines<BufReader<tokio::net::tcp::OwnedReadHalf>>,
    message: &str,
) -> Result<String, Box<dyn Error>> {
    writer.write_all(message.as_bytes()).await?;
    writer.flush().await?;
    Ok(lines.next_line().await?.unwrap_or_default())
}

async fn handle_client(
    socket: tokio::net::TcpStream,
    pool: PgPool,
) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    writer.write_all(b"Welcome to your MUD engine!\r\n").await?;

    // -- Login / Character Creation --
    let name = prompt(&mut writer, &mut lines, "Enter your name: ").await?;

    let player = if let Some(existing_player) = Player::load_from_db(&pool, &name).await? {
        writer.write_all(b"Welcome back!\r\n").await?;
        existing_player
    } else {
        writer.write_all(b"New character detected. Let's create one!\r\n").await?;

        let race = prompt(&mut writer, &mut lines, "Choose a race (human, elf, dwarf): ").await?;
        let class = prompt(&mut writer, &mut lines, "Choose a class (fighter, mage, rogue): ").await?;

        let race_stats = get_race_mods().get(race.as_str()).cloned().unwrap_or_default();
        let class_stats = get_class_mods().get(class.as_str()).cloned().unwrap_or_default();

        let total_stats = StatBlock {
            hp: race_stats.hp + class_stats.hp,
            mana: race_stats.mana + class_stats.mana,
            strength: race_stats.strength + class_stats.strength,
            dexterity: race_stats.dexterity + class_stats.dexterity,
            constitution: race_stats.constitution + class_stats.constitution,
            intelligence: race_stats.intelligence + class_stats.intelligence,
            wisdom: race_stats.wisdom + class_stats.wisdom,
            attacks_per_round: class_stats.attacks_per_round,
        };

        let new_player = Player {
            name: name.clone(),
            race,
            class,
            location: "3001".to_string(),
            inventory: vec![],
            equipped: HashMap::new(),

            hp: total_stats.hp,
            mana: total_stats.mana,
            strength: total_stats.strength,
            dexterity: total_stats.dexterity,
            constitution: total_stats.constitution,
            intelligence: total_stats.intelligence,
            wisdom: total_stats.wisdom,

            level: 1,
            experience: 0,
            max_hp: total_stats.hp,
            max_mana: total_stats.mana,
            gold: 100,
            attacks_per_round: total_stats.attacks_per_round as i16,
        };

        new_player.save_to_db(&pool).await?;
        writer.write_all(b"Character created! Entering the world...\r\n").await?;
        new_player
    };

    // -- Game Context and Command Loop --
    let mut context = GameContext::with_player(player, &pool).await;

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
    context.player.save_to_db(&pool).await?;

    Ok(())
}
