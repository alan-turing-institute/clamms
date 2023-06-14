use serde::{Deserialize, Serialize};

use super::action::Action;

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

        if self.food < 0 {
            f = InvLevel::Critical
        } else if self.food < 10 {
            f = InvLevel::Low
        } else if self.food < 50 {
            f = InvLevel::Medium
        } else {
            f = InvLevel::High
        }

        if self.water < 0 {
            w = InvLevel::Critical
        } else if self.water < 10 {
            w = InvLevel::Low
        } else if self.water < 50 {
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
