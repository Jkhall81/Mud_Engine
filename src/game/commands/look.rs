use crate::game::state::GameContext;
use crate::game::command::Command;

pub struct LookCommand;

impl Command for LookCommand {
    fn execute(&self, _args: &[&str], ctx: &mut GameContext) -> String {
        ctx.describe_current_room()
    }
}
