// Adapted from https://github.com/alan-turing-institute/trustchain/blob/main/trustchain-core/src/config.rs

//! Core configuration types and utilities.
use lazy_static::lazy_static;
// use rand::Error;
use crate::model::action::Action;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use std::fs;
use std::path::Path;
use toml;

pub type ResourceAbundance = f32;

/// Environment variable name for CLAMMS config file.
pub const CLAMMS_CONFIG: &str = "CLAMMS_CONFIG";

lazy_static! {
    /// Lazy static reference to core configuration loaded from `clamms_config.toml`.
    pub static ref CORE_CONFIG: Config = open_config_file(Path::new(std::env::var(CLAMMS_CONFIG).unwrap().as_str()));
}

fn open_config_file(path: &Path) -> Config {
    parse_toml(
        &fs::read_to_string(path).expect(
            format!(
                "Unable to find the file {}. Please check the path is correct and this file exists",
                CLAMMS_CONFIG
            )
            .as_str(),
        ),
    )
    .expect(
        format!(
            "Unable to read the file {}. Please check the contents of this file.",
            CLAMMS_CONFIG
        )
        .as_str(),
    )
}

/// Parses and returns core configuration.
fn parse_toml(toml_str: &str) -> Result<Config, toml::de::Error> {
    toml::from_str::<Config>(toml_str)
}

/// Gets `clamms-core` configuration variables.
pub fn core_config() -> &'static CORE_CONFIG {
    &CORE_CONFIG
}

pub fn degree2radians(deg: f32) -> f32 {
    deg * PI / 180.0
}

pub fn action2rotation(action: Action) -> f32 {
    let degs = match action {
        Action::ToAgent => 180.0,
        Action::Stationary => 0.0,
        Action::ToFood => 0.0,
        Action::ToWater => 0.0,
    };
    degree2radians(degs)
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
    pub FOOD_LOT_SIZE: u32,
    pub WATER_LOT_SIZE: u32,
    pub MAX_TRADE_LOTS: u32,
    pub INVENTORY_LEVEL_CRITICAL_LOW: i32,
    pub INVENTORY_LEVEL_LOW_MEDIUM: i32,
    pub INVENTORY_LEVEL_MEDIUM_HIGH: i32,
    pub DISTANCE_LEVEL_CRITICAL_LOW: u32,
    pub DISTANCE_LEVEL_LOW_MEDIUM: u32,
    pub DISTANCE_LEVEL_MEDIUM_HIGH: u32,
}

/// Configuration variables for `trustchain-core` crate.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WorldConfig {
    /// Config params for simulation world.
    pub N_STEPS: i32,
    pub RANDOM_SEED: u64,
    pub LAND_PROP: f32,
    pub FOOD_ABUNDANCE: f32,
    pub WATER_ABUNDANCE: f32,
    pub TREE_PROB: f32,
    pub SWEET_PROB: f32,
    pub RESOURCE_LOCATIONS_FILE: Option<String>,
    pub WIDTH: u16,
    pub HEIGHT: u16,
    pub N_AGENTS: u8,
    pub HAS_TRADING: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RLConfig {
    pub INIT_Q_VALUES: f32,
    pub SARSA_N: u8,
    pub GAMMA: f32,
    pub ALPHA: f32,
    pub EPSILON: f32,
    pub MULTI_POLICY: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TradeConfig {
    pub MAX_TRADE_DISTANCE: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SimulationConfig {
    pub VERBOSITY: u32,
}
/// Wrapper struct for parsing the `core` table.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    /// Core configuration data.
    pub simulation: SimulationConfig,
    pub agent: AgentConfig,
    pub world: WorldConfig,
    pub trade: TradeConfig,
    pub rl: RLConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let config_string = r##"
        [simulation]
        VERBOSITY = 1

        [world]
        N_STEPS = 100
        RANDOM_SEED = 123
        LAND_PROP = 0.7

        FOOD_ABUNDANCE = 0.1
        WATER_ABUNDANCE = 0.1
        TREE_PROB = 0.1
        SWEET_PROB = 0.01
        N_AGENTS = 10
        WIDTH = 10
        HEIGHT = 10
        HAS_TRADING = true

        [agent]
        INIT_FOOD = 0
        INIT_WATER = 0
        FOOD_ACQUIRE_RATE = 1
        WATER_ACQUIRE_RATE = 1
        FOOD_CONSUME_RATE = 1
        WATER_CONSUME_RATE = 1
        FOOD_MAX_INVENTORY = 456
        WATER_MAX_INVENTORY = 1
        FOOD_LOT_SIZE = 6
        WATER_LOT_SIZE = 2
        MAX_TRADE_LOTS = 1
        INVENTORY_LEVEL_CRITICAL_LOW = 0
        INVENTORY_LEVEL_LOW_MEDIUM = 10
        INVENTORY_LEVEL_MEDIUM_HIGH = 50
        DISTANCE_LEVEL_CRITICAL_LOW = 2
        DISTANCE_LEVEL_LOW_MEDIUM = 10
        DISTANCE_LEVEL_MEDIUM_HIGH = 30

        [trade]
        MAX_TRADE_DISTANCE = 2

        [rl]
        INIT_Q_VALUES = -10000.0
        SARSA_N = 60
        GAMMA = 0.99
        ALPHA = 0.01
        EPSILON = 0.01
        MULTI_POLICY = false
        "##;

        let config: Config = parse_toml(config_string).unwrap();
        println!("config fixture {:?}", config.world);

        assert_eq!(config.world.RANDOM_SEED, 123);
        assert_eq!(config.agent.FOOD_MAX_INVENTORY, 456);
    }

    #[test]
    fn test_malformed_config() {
        let config_missing_params = r##"
        [world]
        RANDOM_SEED = 123

        [agent]
        FOOD_MAX_INVENTORY = 456
        "##;

        let config_extras_params = r##"
        [foo]
        bar = 123
        "##;

        lazy_static! {
            static ref RE: Regex = Regex::new(r"missing field").unwrap();
        }

        for config_string in [config_missing_params, config_extras_params] {
            let actual = parse_toml(config_string);
            assert!(actual.is_err());
            assert!(!actual.is_ok());
            let actual_msg = actual.unwrap_err().to_string();
            println!("test_malformed_config:actual_msg = {actual_msg}");
            assert!(RE.is_match(actual_msg.as_str()));
        }
    }

    #[test]
    #[should_panic]
    fn test_missing_config() {
        open_config_file(Path::new("does_not_exist.fakefile"));
    }
}
