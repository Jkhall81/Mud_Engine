use sqlx::{PgPool};
use crate::game::equipment::{Equipment, EquipmentSlot};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Player {
    pub name: String,
    pub race: String,
    pub class: String,
    pub location: String,

    #[sqlx(skip)]
    pub inventory: Vec<Equipment>,
    #[sqlx(skip)]
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
    pub level: i32,
    pub experience: i32,
    pub max_hp: i32,
    pub max_mana: i32,
    pub gold: i32,
    pub attacks_per_round: i16,
    pub password_hash: String,
}

impl Player {
    pub async fn load_from_db(pool: &PgPool, name: &str) -> Result<Option<Self>, sqlx::Error> {
        let player = sqlx::query_as::<_, Player>(
    r#"
    SELECT
        name, race, class, location,
        hp, mana, strength, dexterity, constitution,
        intelligence, wisdom, level, experience,
        max_hp, max_mana, gold, attacks_per_round,
        password_hash
    FROM players
    WHERE name = $1
    "#
)
.bind(name)
.fetch_optional(pool)
.await?
.map(|mut player| {
    player.inventory = vec![];
    player.equipped = HashMap::new();
    player
});

        // You will still need to load inventory + equipped later if you support it
        Ok(player)
    }

    pub async fn save_to_db(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO players (
                name, race, class, location,
                hp, mana, strength, dexterity, constitution,
                intelligence, wisdom, level, experience,
                max_hp, max_mana, gold, attacks_per_round,
                password_hash
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18)
            ON CONFLICT (name) DO UPDATE SET
                race = EXCLUDED.race,
                class = EXCLUDED.class,
                location = EXCLUDED.location,
                hp = EXCLUDED.hp,
                mana = EXCLUDED.mana,
                strength = EXCLUDED.strength,
                dexterity = EXCLUDED.dexterity,
                constitution = EXCLUDED.constitution,
                intelligence = EXCLUDED.intelligence,
                wisdom = EXCLUDED.wisdom,
                level = EXCLUDED.level,
                experience = EXCLUDED.experience,
                max_hp = EXCLUDED.max_hp,
                max_mana = EXCLUDED.max_mana,
                gold = EXCLUDED.gold,
                attacks_per_round = EXCLUDED.attacks_per_round,
                password_hash = EXCLUDED.password_hash
            "#,
            self.name,
            self.race,
            self.class,
            self.location,
            self.hp,
            self.mana,
            self.strength,
            self.dexterity,
            self.constitution,
            self.intelligence,
            self.wisdom,
            self.level as i32,
            self.experience as i32,
            self.max_hp,
            self.max_mana,
            self.gold as i32,
            self.attacks_per_round as i32,
            self.password_hash,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
