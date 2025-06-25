use crate::game::state::GameContext;

// Only import the trait's dependencies
// Don’t import or re-export commands here directly

pub trait Command: Send + Sync {
    fn execute(&self, args: &[&str], ctx: &mut GameContext) -> String;
}
