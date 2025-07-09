use crate::game::player::Player;
use crate::world::{room::Room, world_loader::World};

pub struct GameContext {
    pub player: Player,
    pub world: World,
}

impl GameContext {
    pub fn with_player(player: Player) -> Self {
        let world = World::load_from_file("assets/rooms.json")
            .expect("Failed to load world");

        Self { player, world }
    }

    pub fn current_room(&self) -> Option<&Room> {
        self.world.rooms.get(&self.player.location)
    }
}
