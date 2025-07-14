use std::collections::HashMap;
use std::sync::Arc;

use crate::game::command::Command;
use crate::game::commands::{LookCommand, MovementCommand, SayCommand, InventoryCommand};
use crate::game::state::GameContext;

pub struct CommandProcessor {
    commands: HashMap<String, Arc<dyn Command + Send + Sync>>,
}

impl Default for CommandProcessor {
    fn default() -> Self {
        let mut commands: HashMap<String, Arc<dyn Command + Send + Sync>> = HashMap::new();

        let look = Arc::new(LookCommand);
        commands.insert("look".into(), look.clone());
        commands.insert("l".into(), look);

        let inventory = Arc::new(InventoryCommand);
        commands.insert("inventory".into(), inventory.clone());
        commands.insert("i".into(), inventory);

        commands.insert("say".into(), Arc::new(SayCommand));

        let dirs = vec![
            ("north", "north"), ("n", "north"),
            ("south", "south"), ("s", "south"),
            ("east",  "east"),  ("e", "east"),
            ("west",  "west"),  ("w", "west"),
            ("up",    "up"),    ("u", "up"),
            ("down",  "down"),  ("d", "down"),
        ];

        for (alias, dir) in dirs {
            commands.insert(alias.into(), Arc::new(MovementCommand { direction: dir.into() }));
        }

        Self { commands }
    }
}

impl CommandProcessor {
    pub fn handle(&self, input: &str, ctx: &mut GameContext) -> String {
        let mut parts = input.split_whitespace();
        let name = parts.next().unwrap_or("").to_lowercase();
        let args: Vec<&str> = parts.collect();

        match self.commands.get(&name) {
            Some(cmd) => cmd.execute(&args, ctx),
            None => "Unknown command.".into(),
        }
    }
}
