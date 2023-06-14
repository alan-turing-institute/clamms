use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::config::core_config;

pub trait DiscrRep<S, L> {
    fn representation(&self) -> ((S, L), (S, L));
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentState {
    pub food: i32,
    pub water: i32,
    // pub last_action: Option<Action>,
}

#[derive(Debug, Clone, PartialEq, EnumIter, Hash, Eq)]
pub enum AgentStateItems {
    Food,
    Water,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentStateDiscrete {
    pub food: InvLevel,
    pub water: InvLevel,
}

impl DiscrRep<AgentStateItems, InvLevel> for AgentState {
    fn representation(&self) -> ((AgentStateItems, InvLevel), (AgentStateItems, InvLevel)) {
        let discr = self.discretise();
        (
            (AgentStateItems::Food, discr.food),
            (AgentStateItems::Water, discr.water),
        )
    }
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

    // pub fn representation<S, L>(&self) -> ((S, L), (S, L))
    // where
    //     S: std::cmp::Eq + std::hash::Hash + Clone,
    //     L: std::cmp::Eq + std::hash::Hash + Clone,
    // {
    //     let discr = self.discretise();
    //     (
    //         (AgentStateItems::Food, discr.food),
    //         (AgentStateItems::Water, discr.water),
    //     )
    // }
}

#[derive(Debug, Clone, PartialEq, EnumIter, Hash, Eq)]
pub enum InvLevel {
    Critical,
    Low,
    Medium,
    High,
}
