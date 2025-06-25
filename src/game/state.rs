use crate::world::room::Room;
use crate::world::world_loader::World;

pub struct GameContext {
    pub player_location: String,
    pub world: World,
}

impl GameContext {
    pub fn new(start_room: String) -> Self {
        let world = World::load_from_file("assets/rooms.json")
            .expect("Failed to load world");

        Self {
            player_location: start_room,
            world,
        }
    }

    pub fn current_room(&self) -> Option<&Room> {
        self.world.rooms.get(&self.player_location)
    }
}
