use serde::{Deserialize, Serialize};

use crate::config::core_config;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentState {
    pub food: i32,
    pub water: i32,
    // pub last_action: Option<Action>,
}

pub struct AgentStateDiscrete {
    pub food: InvLevel,
    pub water: InvLevel,
}

impl AgentState {
    pub fn discretise(&self) -> AgentStateDiscrete {
        let f: InvLevel;
        let w: InvLevel;

        if self.food < core_config().agent.INVENTORY_LEVEL_CRITICAL_LOW {
            f = InvLevel::Critical
        } else if self.food < core_config().agent.INVENTORY_LEVEL_LOW_MEDIUM {
            f = InvLevel::Low
        } else if self.food < core_config().agent.INVENTORY_LEVEL_MEDIUM_HIGH {
            f = InvLevel::Medium
        } else {
            f = InvLevel::High
        }

        if self.water < core_config().agent.INVENTORY_LEVEL_CRITICAL_LOW {
            w = InvLevel::Critical
        } else if self.water < core_config().agent.INVENTORY_LEVEL_LOW_MEDIUM {
            w = InvLevel::Low
        } else if self.water < core_config().agent.INVENTORY_LEVEL_MEDIUM_HIGH {
            w = InvLevel::Medium
        } else {
            w = InvLevel::High
        }

        AgentStateDiscrete { food: f, water: w }
    }
}

pub enum InvLevel {
    Critical,
    Low,
    Medium,
    High,
}
