use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Head,
    NeckOne,
    NeckTwo,
    Chest,
    Shoulders,
    Waist,
    BraceletOne,
    BraceletTwo,
    RingOne,
    RingTwo,
    Legs,
    Feet,
    Hands,
    Shield,
    Weapon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub name: String,
    pub slot: EquipmentSlot,
    pub bonus_attack: i32,
    pub bonus_defense: i32,
}