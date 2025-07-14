use crate::game::state::GameContext;
use crate::game::command::Command;

pub struct InventoryCommand;

impl Command for InventoryCommand {
    fn execute(&self, _args: &[&str], ctx: &mut GameContext) -> String {
        let inventory = &ctx.player.inventory;

        if inventory.is_empty() {
            "Your inventory is empty.".into()
        } else {
            let items: Vec<String> = inventory
                .iter()
                .map(|item| format!("- {}", item.name))
                .collect();

            format!("You are carrying:\r\n{}", items.join("\r\n"))
        }
    }
}