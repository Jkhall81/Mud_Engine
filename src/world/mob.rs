use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mob {
    pub id: String,
    pub name: String,
    pub description: String,

    pub level: u32,
    pub max_hp: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub experience_reward: u32,
    pub gold_reward: u32,
    pub respawn_time: u64,

    pub room_id: String,

    pub is_hostile: bool,
    pub faction: Option<String>,
    pub scripted_behavior: Option<String>,
    pub drops: Option<Vec<ItemDrop>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDrop {
    pub equipment_name: String,     // Reference to equipment.name
    pub drop_chance: f32            // 0.0 to 1.0 (e.g., 0.25 = 25%)
}
