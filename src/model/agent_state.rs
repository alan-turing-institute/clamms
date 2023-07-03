use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use tch::{kind, Tensor};

use crate::config::core_config;

pub trait DiscrRep<S, L> {
    fn representation(&self) -> Vec<(S, L)>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentState {
    pub food: i32,
    pub water: i32,
    pub min_steps_to_food: Option<u32>,
    pub min_steps_to_water: Option<u32>,
    pub min_steps_to_trader: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, EnumIter, Hash, Eq, Serialize, Deserialize)]
pub enum AgentStateItems {
    Food,
    Water,
    MinStepsToFood,
    MinStepsToWater,
    MinStepsToTrader,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentStateDiscrete {
    pub food: InvLevel,
    pub water: InvLevel,
    pub min_steps_to_food: InvLevel,
    pub min_steps_to_water: InvLevel,
    pub min_steps_to_trader: InvLevel,
}

impl DiscrRep<AgentStateItems, InvLevel> for AgentState {
    fn representation(&self) -> Vec<(AgentStateItems, InvLevel)> {
        let discr = self.discretise();

        vec![
            (AgentStateItems::Food, discr.food),
            (AgentStateItems::Water, discr.water),
            (AgentStateItems::MinStepsToFood, discr.min_steps_to_food),
            (AgentStateItems::MinStepsToWater, discr.min_steps_to_water),
            (AgentStateItems::MinStepsToTrader, discr.min_steps_to_trader),
        ]
    }
}

impl AgentState {
    pub fn discretise(&self) -> AgentStateDiscrete {
        let f: InvLevel;
        let w: InvLevel;
        let m_s_f: InvLevel;
        let m_s_w: InvLevel;
        let m_s_t: InvLevel;

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

        if let Some(dist) = self.min_steps_to_food {
            if dist < core_config().agent.DISTANCE_LEVEL_CRITICAL_LOW {
                m_s_f = InvLevel::Critical
            } else if dist < core_config().agent.DISTANCE_LEVEL_LOW_MEDIUM {
                m_s_f = InvLevel::Low
            } else if dist < core_config().agent.DISTANCE_LEVEL_MEDIUM_HIGH {
                m_s_f = InvLevel::Medium
            } else {
                m_s_f = InvLevel::High
            }
        } else {
            m_s_f = InvLevel::High
        }

        if let Some(dist) = self.min_steps_to_water {
            if dist < core_config().agent.DISTANCE_LEVEL_CRITICAL_LOW {
                m_s_w = InvLevel::Critical
            } else if dist < core_config().agent.DISTANCE_LEVEL_LOW_MEDIUM {
                m_s_w = InvLevel::Low
            } else if dist < core_config().agent.DISTANCE_LEVEL_MEDIUM_HIGH {
                m_s_w = InvLevel::Medium
            } else {
                m_s_w = InvLevel::High
            }
        } else {
            m_s_w = InvLevel::High
        }

        if let Some(dist) = self.min_steps_to_trader {
            if dist < core_config().agent.DISTANCE_LEVEL_CRITICAL_LOW {
                m_s_t = InvLevel::Critical
            } else if dist < core_config().agent.DISTANCE_LEVEL_LOW_MEDIUM {
                m_s_t = InvLevel::Low
            } else if dist < core_config().agent.DISTANCE_LEVEL_MEDIUM_HIGH {
                m_s_t = InvLevel::Medium
            } else {
                m_s_t = InvLevel::High
            }
        } else {
            m_s_t = InvLevel::High
        }

        AgentStateDiscrete {
            food: f,
            water: w,
            min_steps_to_food: m_s_f,
            min_steps_to_water: m_s_w,
            min_steps_to_trader: m_s_t,
        }
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

#[derive(Debug, Clone, PartialEq, EnumIter, Hash, Eq, Serialize, Deserialize)]
pub enum InvLevel {
    Critical,
    Low,
    Medium,
    High,
}

impl AgentState {
    pub fn encode(&self) -> Tensor {
        Tensor::from_slice(&[self.food, self.water]).internal_cast_float(true)
    }
}

/// Encodes a slice of `AgentState` to a Tensor.
pub fn encode_vec_of_states(v: &[AgentState]) -> Tensor {
    let v: Vec<Tensor> = v
        .into_iter()
        .map(|agent_state| agent_state.encode())
        .collect();
    Tensor::stack(&v, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::utils::encode_batch;

    #[test]
    fn test_encode() {
        let agent_state = AgentState {
            food: 0,
            water: 1,
            ..Default::default()
        };
        let t1 = agent_state.encode();
        assert_eq!(t1.size(), vec![2]);
        // println!("{}", t1);
    }
    #[test]
    fn test_encode_vec_of_states() {
        let v = vec![
            AgentState {
                food: 0,
                water: 1,
                ..Default::default()
            },
            AgentState {
                food: 4,
                water: 2,
                ..Default::default()
            },
            AgentState {
                food: 9,
                water: 5,
                ..Default::default()
            },
        ];

        let tensor_ts1 = encode_vec_of_states(&v);
        let tensor_ts2 = encode_vec_of_states(&v);
        let batch_ts = encode_batch(&[tensor_ts1, tensor_ts2]);

        assert_eq!(batch_ts.size(), vec![2, 3, 2]);
        // println!("{}", t1);
    }

    // Sample 1: [0, 1], [2, 3], [4, 5] : What shape is this? (3, 2) |
    // Sample 2: [6, 7], [8, 9], [10, 11]                            |
    // -> Shape: (2, 3, 2)

    // #[test]
    // fn test_encode() {
    //     let agent_state = AgentState {food: 0, water: 1};
    //     let t1 = agent_state.encode();

    //     let t1_2 = agent_state.encode();
    //     let t2 = agent_state.encode();
    //     let t2_2 = agent_state.encode();
    //     let t3 = agent_state.encode().reshape([1, 2]);
    //     let t_stack = tch::Tensor::vstack(&[t1, t2]);
    //     let t3 = tch::Tensor::vstack(&[t1_2, t2_2]);
    //     let t_stack_again = tch::Tensor::stack(&[t_stack, t3], 0);
    //     // println!("{}", t1_2);
    //     // println!("{}", t_stack);
    //     println!("{}", t_stack_again);
    // }
}
