use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct StatBlock {
    pub hp: i32,
    pub mana: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub attacks_per_round: u8,
}

pub fn get_race_mods() -> HashMap<&'static str, StatBlock> {
    HashMap::from([
        ("human", StatBlock { hp: 10, mana: 10, strength: 1, dexterity: 1, constitution: 1, intelligence: 1, wisdom: 1, attacks_per_round: 0 }),
        ("elf", StatBlock { hp: 8, mana: 12, strength: 0, dexterity: 2, constitution: 0, intelligence: 2, wisdom: 2, attacks_per_round: 0 }),
        ("dwarf", StatBlock { hp: 12, mana: 6, strength: 2, dexterity: 0, constitution: 3, intelligence: 0, wisdom: 1, attacks_per_round: 0 }),
    ])
}

pub fn get_class_mods() -> HashMap<&'static str, StatBlock> {
    HashMap::from([
        ("fighter", StatBlock { hp: 10, mana: 0, strength: 2, dexterity: 1, constitution: 2, intelligence: 0, wisdom: 0, attacks_per_round: 2 }),
        ("mage", StatBlock { hp: 4, mana: 15, strength: 0, dexterity: 1, constitution: 0, intelligence: 3, wisdom: 2, attacks_per_round: 1 }),
        ("rogue", StatBlock { hp: 6, mana: 4, strength: 1, dexterity: 3, constitution: 1, intelligence: 1, wisdom: 0, attacks_per_round: 1 }),
    ])
}