use serde::{Deserialize, Serialize};
use tch::Tensor;

//   one_hot(3) -> tensor of shape [3, 3]


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    ToFood,
    ToWater,
    Stationary,
}

impl Action {
    pub fn encode(&self) -> Tensor {
        match self {
            Action::ToFood => Tensor::from_slice(&[1, 0, 0]).internal_cast_float(true),
            Action::ToWater => Tensor::from_slice(&[0, 1, 0]).internal_cast_float(true),
            Action::Stationary => Tensor::from_slice(&[0, 0, 1]).internal_cast_float(true),
        }  
    } 
}