use crate::game::state::GameContext;
use crate::game::command::Command;

pub struct LookCommand;

impl Command for LookCommand {
    fn execute(&self, _args: &[&str], ctx: &mut GameContext) -> String {
        match ctx.current_room() {
            Some(room) => room.description.clone(),
            None => "You are in a void.".into(),
        }
    }
}
