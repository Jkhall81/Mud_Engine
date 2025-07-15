use crate::game::state::GameContext;
use crate::game::command::Command;

pub struct ScoreCommand;

impl Command for ScoreCommand {
    fn execute(&self, _args: &[&str], ctx: &mut GameContext) -> String {
        let player = &ctx.player;
                format!(
            "\
Name: {}\r\n\
Race: {}\r\n\
Class: {}\r\n\
Level: {}\r\n\
XP: {}\r\n\
HP: {}/{}\r\n\
Mana: {}/{}\r\n\
Gold: {}\r\n\
Location: {}\r\n\
Attacks per round: {}\r\n",
            player.name,
            player.race,
            player.class,
            player.level,
            player.experience,
            player.hp,
            player.max_hp,
            player.mana,
            player.max_mana,
            player.gold,
            player.location,
            player.attacks_per_round
        )
    }
}