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

    // Core stats
    pub hp: i32,
    pub mana: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,

    // New Fields
    pub level: u32,
    pub experience: u32,
    pub max_hp: i32,
    pub max_mana: i32,
    pub gold: u32,
    pub attacks_per_round: u8,
}

    impl Player {
        pub fn save_to_file(&self) -> std::io::Result<()> {
        let dir = std::path::Path::new("assets/players");
        std::fs::create_dir_all(dir)?; // Make sure the folder exists

        let path = dir.join(format!("{}.json", self.name.to_lowercase()));
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)
    }


    pub fn load_from_file(name: &str) -> std::io::Result<Self> {
        let path = format!("assets/players/{}.json", name.to_lowercase());
        let data = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }
}