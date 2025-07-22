use crate::world::room::Room;
use sqlx::PgPool;
use std::fs;

pub async fn load_rooms_from_dir(pool: &PgPool, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut room_list = Vec::new();

    // First pass: parse all rooms
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let contents = fs::read_to_string(&path)?;
            let room: Room = serde_json::from_str(&contents)?;
            room_list.push(room);
        }
    }

    // Second pass: insert all rooms
    for room in &room_list {
        sqlx::query!(
            r#"
            INSERT INTO rooms (id, title, description)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET
                title = EXCLUDED.title,
                description = EXCLUDED.description
            "#,
            room.id,
            room.title,
            room.description
        )
        .execute(pool)
        .await?;
    }

    // Third pass: insert all exits
    for room in &room_list {
        for (dir, exit) in &room.exits {
            sqlx::query!(
                r#"
                INSERT INTO room_exits (from_room, direction, to_room)
                VALUES ($1, $2, $3)
                ON CONFLICT (from_room, direction) DO UPDATE SET to_room = EXCLUDED.to_room
                "#,
                room.id,
                dir,
                exit.to
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
