use std::collections::HashMap;
use sqlx::PgPool;
use crate::world::room::{Room, RoomExit};

#[derive(Debug)]
pub struct World {
    pub rooms: HashMap<String, Room>,
}

impl World {
    pub async fn load_from_db(pool: &PgPool) -> Result<Self, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, description
            FROM rooms
            "#
        )
        .fetch_all(pool)
        .await?;

        let mut rooms = HashMap::new();
        for row in rows {
            // You may need to populate other fields here if your Room struct has more.
            rooms.insert(
                row.id.clone(),
                Room {
                    id: row.id,
                    description: row.description,
                    ..Default::default() // Requires Room to implement Default
                },
            );
        }

        // Now fetch exits
        let exits = sqlx::query!(
            r#"
            SELECT from_room, direction, to_room::TEXT as to_room
            FROM room_exits
            "#
        )
        .fetch_all(pool)
        .await?;

       for exit in exits {
            if let Some(room) = rooms.get_mut(&exit.from_room) {
              room.exits.insert(
                exit.direction,
                RoomExit {
                    to: exit.to_room.expect("to_room should not be NULL"),
                    description: String::new(),
                    is_door: false,
                    is_locked: false,
                },
            );
            }
        }
        Ok(World { rooms })
    }
}
