use crate::game::state::GameContext;
use crate::game::command::Command;

pub struct MovementCommand {
    pub direction: String,
}

impl Command for MovementCommand {
    fn execute(&self, _args: &[&str], ctx: &mut GameContext) -> String {
        let dir = &self.direction;
        let current_room = match ctx.current_room() {
            Some(room) => room,
            None => return "You are lost in the void.".into(),
        };

        match current_room.exits.get(dir) {
            Some(exit) => {
                let next_room_id = &exit.to;
                if ctx.world.rooms.contains_key(next_room_id) {
                    ctx.player.location = next_room_id.clone();
                    ctx.describe_current_room()
                } else {
                    "That direction leads to nothing.".into()
                }
            }
            None => "You can't go that way.".into(),
        }
    }
}
