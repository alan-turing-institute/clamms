use serde::{Deserialize, Serialize};

use super::action::Action;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentState {

    pub food: i32,
    pub water: i32,
    pub min_steps_to_food: Option<u32>,
    pub min_steps_to_water: Option<u32>,
    pub min_steps_to_trader: Option<u32>,

    // OLD:
    // pub last_action: Option<Action>,
}
