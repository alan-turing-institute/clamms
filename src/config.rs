// Adapted from https://github.com/alan-turing-institute/trustchain/blob/main/trustchain-core/src/config.rs

//! Core configuration types and utilities.
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

/// Environment variable name for CLAMMS config file.
pub const CLAMMS_CONFIG: &str = "CLAMMS_CONFIG";

lazy_static! {
    /// Lazy static reference to core configuration loaded from `clamms_config.toml`.
    pub static ref CORE_CONFIG: Config = parse_toml(
        &fs::read_to_string(std::env::var(CLAMMS_CONFIG).unwrap().as_str())
        .expect("Error reading clamms_config.toml"));
}

/// Parses and returns core configuration.
fn parse_toml(toml_str: &str) -> Config {
    toml::from_str::<Config>(toml_str).expect("Error parsing clamms_config.toml")
}

/// Gets `clamms-core` configuration variables.
pub fn core_config() -> &'static CORE_CONFIG {
    &CORE_CONFIG
}

/// Configuration variables for `trustchain-core` crate.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AgentConfig {
    /// Config param for Agent
    pub INIT_FOOD: i32,
    pub INIT_WATER: i32,
    pub FOOD_ACQUIRE_RATE: i32,
    pub WATER_ACQUIRE_RATE: i32,
    pub FOOD_CONSUME_RATE: u32,
    pub WATER_CONSUME_RATE: u32,
    pub FOOD_MAX_INVENTORY: i32,
    pub WATER_MAX_INVENTORY: i32,
}

/// Configuration variables for `trustchain-core` crate.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WorldConfig {
    /// Config params for simulation world.
    pub RANDOM_SEED: u32,
    pub FOOD_ABUNDANCE: f32,
    pub WATER_ABUNDANCE: f32,
    pub TREE_PROB: f32,
    pub SWEET_PROB: f32,
}

/// Wrapper struct for parsing the `core` table.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    /// Core configuration data.
    pub agent: AgentConfig,
    pub world: WorldConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let config_string = r##"
        [world]
        random_seed = 123;
        "##;

        let config: Config = parse_toml(config_string);
        println!("config fixture {:?}", config.world);

        assert_eq!(config.world.RANDOM_SEED, 123);
    }
}
