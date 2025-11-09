// @generated automatically by Diesel CLI.

diesel::table! {
    player_config (id) {
        id -> BigInt,
        skip_upgrade_prompt -> Bool,
    }
}

diesel::table! {
    player_data (id) {
        id -> BigInt,
        health -> Double,
        max_health -> Double,
        last_attack -> BigInt,
        total_damage -> Double,
        available_honors -> Double,
    }
}

diesel::table! {
    player_stats (id) {
        id -> BigInt,
        attack_success_level -> Integer,
        attack_flat_level -> Integer,
        attack_mult_level -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(player_config, player_data, player_stats,);
