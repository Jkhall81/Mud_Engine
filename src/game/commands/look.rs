use crate::game::state::GameContext;
use crate::game::command::Command;

pub struct LookCommand;

impl Command for LookCommand {
    fn execute(&self, _args: &[&str], ctx: &mut GameContext) -> String {
        match ctx.current_room() {
            Some(room) => {
                let mut output = format!("{}", room.description);
                if !room.exits.is_empty() {
                    let directions: Vec<&str> = room.exits.keys().map(|s| s.as_str()).collect();
                    let exits = directions.join(", ");
                    output.push_str(&format!("\r\nExits: {}", exits));
                } else {
                    output.push_str("\r\nThere are no visible exits.");
                }
                output
            }
            None => "You are in a void.".into(),
        }
    }
}
