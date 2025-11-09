-- Your SQL goes here
CREATE TABLE player_data (
  id BIGINT NOT NULL PRIMARY KEY,
  health DOUBLE NOT NULL DEFAULT 5.0,
  max_health DOUBLE NOT NULL DEFAULT 5.0,
  last_attack BIGINT NOT NULL DEFAULT 0,
  total_damage DOUBLE NOT NULL DEFAULT 0.0,
  available_honors DOUBLE NOT NULL DEFAULT 0.0
)