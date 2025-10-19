// @generated automatically by Diesel CLI.

diesel::table! {
    player_data (id) {
        id -> BigInt,
        health -> Double,
        max_health -> Double,
        last_attack -> BigInt,
        total_damage -> Double,
    }
}
