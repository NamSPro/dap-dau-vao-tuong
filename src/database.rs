use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

use crate::models::PlayerData;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migrations() -> Result<(), crate::Error> {
    let conn = &mut establish_connection();
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn get_player_data(user_id: i64) -> QueryResult<PlayerData> {
    use crate::schema::player_data::dsl::*;
    let conn = &mut establish_connection();
    let results = player_data
        .filter(id.eq(user_id))
        .select(PlayerData::as_select())
        .load(conn)
        .expect("Error loading player data");
    if results.is_empty() {
        let new_player = PlayerData {
            id: user_id,
            health: 5.0,
            max_health: 5.0,
            last_attack: 0,
            total_damage: 0.0,
        };
        diesel::insert_into(player_data).values(&new_player).execute(conn)?;
        Ok(new_player)
    } else {
        Ok(results.into_iter().next().unwrap())
    }
}

pub fn set_player_data(player: &PlayerData) -> QueryResult<()> {
    use crate::schema::player_data::dsl::*;
    let conn = &mut establish_connection();
    diesel::replace_into(player_data)
        .values(player)
        .execute(conn)?;
    Ok(())
}