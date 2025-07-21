use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::error::Error;
use std::collections::HashMap;
use sqlx::PgPool;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng, Error as ArgonError};

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

fn hash_password(password: &str) -> Result<String, ArgonError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, ArgonError> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

async fn handle_client(
    socket: tokio::net::TcpStream,
    pool: PgPool,
) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    writer.write_all(b"Welcome to your MUD engine!\r\n").await?;

    loop {
        writer.write_all(b"\r\n1. Create account\r\n2. Log in\r\n3. Change password\r\n4. Delete account\r\n5. Exit\r\n> ").await?;
        if let Some(choice) = lines.next_line().await? {
            match choice.trim() {
                "1" => return handle_create_account(&mut writer, &mut lines, &pool).await,
                "2" => return handle_login(&mut writer, &mut lines, &pool).await,
                "3" => handle_change_password(&mut writer, &mut lines, &pool).await?,
                "4" => handle_delete_account(&mut writer, &mut lines, &pool).await?,
                "5" => {
                    writer.write_all(b"Goodbye!\r\n").await?;
                    return Ok(());
                }
                _ => writer.write_all(b"Invalid option.\r\n").await?,
            }
        } else {
            break;
        }
    }

    Ok(())
}

async fn handle_create_account(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    lines: &mut tokio::io::Lines<BufReader<tokio::net::tcp::OwnedReadHalf>>,
    pool: &PgPool,
) -> Result<(), Box<dyn Error>> {
    let name = prompt(writer, lines, "Choose a username: ").await?;
    if Player::load_from_db(pool, &name).await?.is_some() {
        writer.write_all(b"Name already exists.\r\n").await?;
        return Ok(());
    }

    let password = prompt(writer, lines, "Choose a password: ").await?;
    let confirm = prompt(writer, lines, "Confirm password: ").await?;

    if password != confirm {
        writer.write_all(b"Passwords do not match.\r\n").await?;
        return Ok(());
    }

    let password_hash = hash_password(&password).map_err(|e| format!("{}", e))?;
    writer.write_all(b"Creating character...\r\n").await?;

    let race = prompt(writer, lines, "Choose a race (human, elf, dwarf): ").await?;
    let class = prompt(writer, lines, "Choose a class (fighter, mage, rogue): ").await?;

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
        password_hash,
    };

    new_player.save_to_db(pool).await?;
    writer.write_all(b"Character created! Entering the world...\r\n").await?;
    start_game(writer, lines, new_player, pool).await
}

async fn handle_login(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    lines: &mut tokio::io::Lines<BufReader<tokio::net::tcp::OwnedReadHalf>>,
    pool: &PgPool,
) -> Result<(), Box<dyn Error>> {
    let name = prompt(writer, lines, "Enter username: ").await?;
    if let Some(player) = Player::load_from_db(pool, &name).await? {
        let password = prompt(writer, lines, "Enter password: ").await?;
        if verify_password(&password, &player.password_hash).map_err(|e| format!("{}", e))? {
            writer.write_all(b"Welcome back!\r\n").await?;
            return start_game(writer, lines, player, pool).await;
        } else {
            writer.write_all(b"Invalid password.\r\n").await?;
        }
    } else {
        writer.write_all(b"Player not found.\r\n").await?;
    }
    Ok(())
}

async fn start_game(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    lines: &mut tokio::io::Lines<BufReader<tokio::net::tcp::OwnedReadHalf>>,
    player: Player,
    pool: &PgPool,
) -> Result<(), Box<dyn Error>> {
    let mut context = GameContext::with_player(player, pool).await;
    let processor = CommandProcessor::default();
    let intro = context.describe_current_room();
    writer.write_all(intro.as_bytes()).await?;
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

    context.player.save_to_db(pool).await?;
    Ok(())
}

async fn handle_change_password(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    lines: &mut tokio::io::Lines<BufReader<tokio::net::tcp::OwnedReadHalf>>,
    pool: &PgPool,
) -> Result<(), Box<dyn Error>> {
    let name = prompt(writer, lines, "Enter username: ").await?;
    if let Some(mut player) = Player::load_from_db(pool, &name).await? {
        let current = prompt(writer, lines, "Enter current password: ").await?;
        if verify_password(&current, &player.password_hash).map_err(|e| format!("{}", e))? {
            let new_pass = prompt(writer, lines, "Enter new password: ").await?;
            let confirm = prompt(writer, lines, "Confirm new password: ").await?;
            if new_pass == confirm {
                player.password_hash = hash_password(&new_pass).map_err(|e| format!("{}", e))?;
                player.save_to_db(pool).await?;
                writer.write_all(b"Password changed successfully.\r\n").await?;
            } else {
                writer.write_all(b"Passwords do not match.\r\n").await?;
            }
        } else {
            writer.write_all(b"Invalid current password.\r\n").await?;
        }
    } else {
        writer.write_all(b"Player not found.\r\n").await?;
    }
    Ok(())
}

async fn handle_delete_account(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    lines: &mut tokio::io::Lines<BufReader<tokio::net::tcp::OwnedReadHalf>>,
    pool: &PgPool,
) -> Result<(), Box<dyn Error>> {
    let name = prompt(writer, lines, "Enter username: ").await?;
    if let Some(player) = Player::load_from_db(pool, &name).await? {
        let password = prompt(writer, lines, "Enter password to confirm deletion: ").await?;
        if verify_password(&password, &player.password_hash).map_err(|e| format!("{}", e))? {
            sqlx::query("DELETE FROM players WHERE name = $1")
                .bind(&name)
                .execute(pool)
                .await?;
            writer.write_all(b"Account deleted.\r\n").await?;
        } else {
            writer.write_all(b"Invalid password.\r\n").await?;
        }
    } else {
        writer.write_all(b"Player not found.\r\n").await?;
    }
    Ok(())
}
