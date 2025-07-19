use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Room {
    pub id: String,
    pub title: String,
    pub description: String,
    pub zone: u16,
    pub flags: Vec<String>,
    pub sector: String,
    pub exits: HashMap<String, RoomExit>,

    #[serde(default)]
    pub extra_descriptions: Vec<ExtraDescription>,

    #[serde(default)]
    pub triggers: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomExit {
    pub to: String,
    pub description: String,
    pub is_door: bool,
    pub is_locked: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtraDescription {
    pub keyword: String,
    pub description: String
}

impl Default for Room {
    fn default() -> Self {
        Room {
            id: String::new(),
            title: String::new(),
            description: String::new(),
            zone: 0,
            flags: Vec::new(),
            sector: String::new(),
            exits: HashMap::new(),
            extra_descriptions: Vec::new(),
            triggers: Vec::new(),
        }
    }
}
