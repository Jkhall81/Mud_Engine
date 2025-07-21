use crate::game::equipment::{Equipment};
use sqlx::PgPool;
use std::fs;

pub async fn load_equipment_from_dir(pool: &PgPool, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let contents = fs::read_to_string(&path)?;
            let item: Equipment = serde_json::from_str(&contents)?;

            let slot_str = format!("{:?}", item.slot);

            sqlx::query!(
                r#"
                INSERT INTO equipment (name, slot, bonus_attack, bonus_defense)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (name) DO UPDATE
                SET slot = EXCLUDED.slot,
                    bonus_attack = EXCLUDED.bonus_attack,
                    bonus_defense = EXCLUDED.bonus_defense
                "#,
                item.name,
                slot_str,
                item.bonus_attack,
                item.bonus_defense
            )
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}