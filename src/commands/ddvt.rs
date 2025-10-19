use chrono::Local;

use crate::models::PlayerData;

pub fn regen(mut player: PlayerData) -> PlayerData {
    let now = Local::now().timestamp();
    let time_diff = now - player.last_attack;
    let regen_amount = time_diff / 60; // Regenerate 1 HP every 60 seconds
    if player.health <= 0.0 {
        if regen_amount as f64 >= 2.0 * player.max_health {
            player.health = player.max_health;
        }
        return player;
    }
    if regen_amount > 0 {
        player.health += regen_amount as f64;
        if player.health > player.max_health {
            player.health = player.max_health;
        }
    }
    player
}

pub fn attack(mut player: PlayerData, damage_dealt: f64, damage_taken: f64) -> PlayerData {
    if player.health <= 0.0 {
        return player;
    }
    player.health -= damage_taken;
    if player.health < 0.0 {
        player.health = 0.0;
    }
    player.total_damage += damage_dealt;
    player.last_attack = Local::now().timestamp();
    player
}