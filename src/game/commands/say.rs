use crate::game::state::GameContext;
use crate::game::command::Command;

pub struct SayCommand;

impl Command for SayCommand {
    fn execute(&self, args: &[&str], _ctx: &mut GameContext) -> String {
        if args.is_empty() {
            return "Say what?".into();
        }

        let message = args.join(" ");
        format!("You say: \"{}\"", message)
    }
}
