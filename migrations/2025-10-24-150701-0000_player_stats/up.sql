-- Your SQL goes here
CREATE TABLE player_stats (
  id BIGINT NOT NULL PRIMARY KEY,
  attack_success_level INT NOT NULL DEFAULT 0,
  attack_flat_level INT NOT NULL DEFAULT 1,
  attack_mult_level INT NOT NULL DEFAULT 0
);

CREATE TABLE player_config (
  id BIGINT NOT NULL PRIMARY KEY,
  skip_upgrade_prompt BOOLEAN NOT NULL DEFAULT FALSE
);