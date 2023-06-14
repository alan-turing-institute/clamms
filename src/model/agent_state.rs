use serde::{Deserialize, Serialize};
use tch::{Tensor, kind};

use super::action::Action;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentState {
    pub food: i32,
    pub water: i32,
    // pub last_action: Option<Action>,
}

impl AgentState {
   pub fn encode(&self) -> Tensor {
        Tensor::from_slice(&[self.food, self.water]).internal_cast_float(true)
    }    
}

/// Encodes a slice of `AgentState` to a Tensor.
pub fn encode_vec_of_states(v: &[AgentState]) -> Tensor {
    let v: Vec<Tensor> = v.into_iter().map(|agent_state| agent_state.encode()).collect();
    Tensor::stack(&v, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::utils::encode_batch;
    
    #[test]
    fn test_encode() {
        let agent_state = AgentState {food: 0, water: 1};
        let t1 = agent_state.encode();
        assert_eq!(t1.size(), vec![2]);
        // println!("{}", t1);
        
    }
    #[test]
    fn test_encode_vec_of_states() {
        let v = vec![AgentState {food: 0, water: 1}, 
        AgentState {food: 4, water: 2},
        AgentState {food: 9, water: 5}];

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