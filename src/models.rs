use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::player_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PlayerData {
    pub id: i64,
    pub health: f64,
    pub max_health: f64,
    pub last_attack: i64, // epoch time
    pub total_damage: f64,
    pub available_honors: f64,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::player_stats)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PlayerStats {
    pub id: i64,
    pub attack_success_level: i32,
    pub attack_flat_level: i32,
    pub attack_mult_level: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::player_config)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PlayerConfig {
    pub id: i64,
    pub skip_upgrade_prompt: bool,
}