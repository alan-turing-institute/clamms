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
    fn encode(&self) -> Tensor {
        let t = Tensor::zeros(&[2], kind::FLOAT_CPU);
        // t = torch.tensor([food, water])
        let t = Tensor::from_slice(&[self.food, self.water]).internal_cast_float(true);
        // torch.cast(t, FLOAT_CPU)
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let agent_state = AgentState {food: 0, water: 0};
        let t = agent_state.encode();
        println!("{}", t);
    }
}