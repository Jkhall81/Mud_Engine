use crate::game::equipment::{Equipment, EquipmentSlot};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub name: String,
    pub race: String,
    pub class: String,
    pub location: String,
    pub inventory: Vec<Equipment>,
    pub equipped: HashMap<EquipmentSlot, Equipment>,
}

impl Player {
    pub fn save_to_file(&self) -> std::io::Result<()> {
        let path = format!("assets/players/{}.json", self.name.to_lowercase());
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)
    }

    pub fn load_from_file(name: &str) -> std::io::Result<Self> {
        let path = format!("assets/players/{}.json", name.to_lowercase());
        let data = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }
}