use std::sync::Once;

use crate::config::CLAMMS_CONFIG;

pub mod action;
pub mod agent_api;
pub mod agent_state;
pub mod board;
pub mod environment;
pub mod forager;
pub mod history;
pub mod inventory;
pub mod lstm_nn;
pub mod policy;
pub mod q_table;
pub mod reward;
pub mod routing;
pub mod tabular_rl;
pub mod trader;
pub mod utils;

static INIT: Once = Once::new();
pub fn init() {
    INIT.call_once(|| {
        std::env::set_var(CLAMMS_CONFIG, std::env::var("CLAMMS_CONFIG_TEST").unwrap());
    });
}
