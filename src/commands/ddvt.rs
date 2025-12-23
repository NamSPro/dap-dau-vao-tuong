use chrono::Local;
use crate::database;

const ATTACK_SUCCESS_BASE: f64 = 0.5;
const ATTACK_BASE: f64 = 1.0;
const ATTACK_MULT: f64 = 1.05;
const DAMAGE_BASE: f64 = 1.0;

pub fn regen(user: i64) -> Result<(f64, f64, i64), crate::Error> {
    let mut player = database::get_player_data(user)?;
    let now = Local::now().timestamp();
    let time_diff = now - player.last_attack;
    let regen_amount = time_diff / 60; // Regenerate 1 HP every 60 seconds
    if player.health <= 0.0 {
        if regen_amount as f64 >= 2.0 * player.max_health {
            player.health = player.max_health;
        }
        database::set_player_data(&player)?;
        return Ok((player.health, player.max_health, player.last_attack));
    }
    if regen_amount > 0 {
        player.health += regen_amount as f64;
        if player.health > player.max_health {
            player.health = player.max_health;
        }
    }
    player.last_attack = player.last_attack + regen_amount * 60;
    database::set_player_data(&player)?;
    Ok((player.health, player.max_health, player.last_attack))
}

pub fn attack(user: i64) -> Result<(bool, f64, f64), crate::Error> {
    let mut player = database::get_player_data(user)?;
    let stats = database::get_player_stats(user)?;
    if player.health <= 0.0 {
        return Ok((false, 0.0, 0.0));
    }
    player.health -= DAMAGE_BASE;
    if player.health < 0.0 {
        player.health = 0.0;
    }
    let roll = rand::random_bool(ATTACK_SUCCESS_BASE + 0.01 * stats.attack_success_level as f64);
    let mut damage_dealt = ATTACK_BASE * stats.attack_flat_level as f64 * ATTACK_MULT.powi(stats.attack_mult_level);
    if !roll { 
        damage_dealt *= 0.5; 
    }
    player.total_damage += damage_dealt;
    player.available_honors += damage_dealt;
    player.last_attack = Local::now().timestamp();
    database::set_player_data(&player)?;
    Ok((roll, damage_dealt, player.health))
}

pub fn get_total_damage(user: i64) -> Result<f64, crate::Error> {
    let player = database::get_player_data(user)?;
    Ok(player.total_damage)
}