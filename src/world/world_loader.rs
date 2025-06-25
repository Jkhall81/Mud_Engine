use std::fs;
use std::collections::HashMap;
use serde::Deserialize;

use crate::world::room::Room;

#[derive(Debug, Deserialize)]
pub struct World {
    pub rooms: HashMap<String, Room>,
}

impl World {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path)?;
        let world: World = serde_json::from_str(&data)?;
        Ok(world)
    }
}
