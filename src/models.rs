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
}
