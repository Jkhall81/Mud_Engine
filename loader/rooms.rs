use crate::world::room::Room;
use sqlx::PgPool;
use std::fs;
use std::path::Path;

pub async fn load_rooms_from_dir(pool: &PgPool, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let contents = fs::read_to_string(&path)?;
            let room: Room = serde_json::from_str(&contents)?;

            // Use room.id from JSON
            sqlx::query!(
                r#"
                INSERT INTO rooms (id, description)
                VALUES ($1, $2)
                ON CONFLICT (id) DO UPDATE SET description = EXCLUDED.description
                "#,
                room.id,
                room.description
            )
            .execute(pool)
            .await?;

            for (dir, to_room) in room.exits {
                sqlx::query!(
                    r#"
                    INSERT INTO room_exits (from_room, direction, to_room)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (from_room, direction) DO UPDATE SET to_room = EXCLUDED.to_room
                    "#,
                    room.id,
                    dir,
                    to_room
                )
                .execute(pool)
                .await?;
            }
        }
    }

    Ok(())
}


pub async fn validate_room_links(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let missing = sqlx::query!(
        r#"
        SELECT from_room, direction, to_room
        FROM room_exits
        WHERE to_room NOT IN (SELECT id FROM rooms)
        "#
    )
    .fetch_all(pool)
    .await?;

    if missing.is_empty() {
        println!("All room links are valid.");
    } else {
        println!("Missing target rooms:");
        for row in missing {
            println!(
                "  [{}] {} -> {} (MISSING)",
                row.from_room, row.direction, row.to_room
            );
        }
    }

    Ok(())
}
