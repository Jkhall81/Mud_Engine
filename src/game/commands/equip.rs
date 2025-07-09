use crate::game::state::GameContext;
use crate::game::command::Command;
use crate::game::equipment::{Equipment, EquipmentSlot};

pub struct EquipCommand;

impl Command for EquipCommand {
    fn execute(&self, args: &[&str], ctx: &mut GameContext) -> String {
        if args.is_empty() {
            return "Equip what?".into();
        }

        let item_name = args.join(" ").to_lowercase();
        let inventory = &mut ctx.player.inventory;

        if let Some(pos) = inventory.iter().position(|item| item.name.to_lowercase() == item_name) {
            let item = inventory.remove(pos);
            let slot = item.slot.clone();

            if let Some(prev) = ctx.player.equipped.insert(slot.clone(), item.clone()) {
                inventory.push(prev);
                format!(
                    "You equip the {} and remove the {} from your {} slot.",
                    item.name, item.name, format!("{:?}", slot).to_lowercase()
                )
            } else {
                format!("You equip the {} to your {} slot.", item.name, format!("{:?}", slot).to_lowercase())
            }
        } else {
            format!("You don't have a '{}' to equip.", item_name)
        }
    }
}
