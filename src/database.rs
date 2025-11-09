use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

use crate::{models::{PlayerData, PlayerStats}, Error};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migrations() -> Result<(), Error> {
    let conn = &mut establish_connection();
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn get_player_data(user_id: i64) -> Result<PlayerData, Error> {
    use crate::schema::player_data::dsl::*;
    let conn = &mut establish_connection();
    let results = player_data
        .filter(id.eq(user_id))
        .select(PlayerData::as_select())
        .load(conn)
        .expect("Error loading player data");
    if results.is_empty() {
        diesel::insert_into(player_data)
            .values(&Some(id.eq(user_id)))
            .execute(conn)?;
        get_player_data(user_id)
    } else {
        Ok(results.into_iter().next().unwrap())
    }
}

pub fn set_player_data(player: &PlayerData) -> Result<(), Error> {
    use crate::schema::player_data::dsl::*;
    let conn = &mut establish_connection();
    diesel::replace_into(player_data)
        .values(player)
        .execute(conn)?;
    Ok(())
}

pub fn get_player_stats(user_id: i64) -> Result<PlayerStats, Error> {
    use crate::schema::player_stats::dsl::*;
    let conn = &mut establish_connection();
    let results = player_stats
        .filter(id.eq(user_id))
        .select(crate::models::PlayerStats::as_select())
        .load(conn)
        .expect("Error loading player stats");
    if results.is_empty() {
        diesel::insert_into(player_stats)
            .values(&Some(id.eq(user_id)))
            .execute(conn)?;
        get_player_stats(user_id)
    } else {
        Ok(results.into_iter().next().unwrap())
    }
}