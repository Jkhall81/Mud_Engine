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
}
