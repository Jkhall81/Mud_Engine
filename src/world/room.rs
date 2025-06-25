use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Room {
    pub id: String,
    pub description: String,
    pub exits: HashMap<String, String>,
}
