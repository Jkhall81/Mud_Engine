use crate::game::player::Player;
use crate::world::{room::Room, world_loader::World};
use sqlx::PgPool;

pub struct GameContext {
    pub player: Player,
    pub world: World,
}

impl GameContext {
    pub async fn with_player(player: Player, pool: &PgPool) -> Self {
        let world = World::load_from_db(pool)
            .await
            .expect("Failed to load world from DB");

        Self { player, world }
    }

    pub fn current_room(&self) -> Option<&Room> {
        self.world.rooms.get(&self.player.location)
    }

     pub fn describe_current_room(&self) -> String {
        if let Some(room) = self.current_room() {
            let exits = if room.exits.is_empty() {
                "None".to_string()
            } else {
                room.exits.keys().cloned().collect::<Vec<_>>().join(", ")
            };

            format!(
                "{}\r\n{}\r\nExits: {}\r\n",
                &room.title,
                room.description,
                exits
            )
        } else {
            "You are in an unknown void.\r\n".to_string()
        }
    }
}
