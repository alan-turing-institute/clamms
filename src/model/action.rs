use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};
use tch::Tensor;

//   one_hot(3) -> tensor of shape [3, 3]

use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, Hash, Eq)]
pub enum Action {
    ToFood,
    ToWater,
    ToAgent,
    Stationary,
}

impl Action {
    pub fn encode(&self) -> Tensor {
        match self {
            Action::ToFood => Tensor::from_slice(&[1, 0, 0, 0]).internal_cast_float(true),
            Action::ToWater => Tensor::from_slice(&[0, 1, 0, 0]).internal_cast_float(true),
            Action::Stationary => Tensor::from_slice(&[0, 0, 1, 0]).internal_cast_float(true),
            Action::ToAgent => Tensor::from_slice(&[0, 0, 0, 1]).internal_cast_float(true),
        }
    }
}

/// Encodes a slice of `Action` to a Tensor.
pub fn encode_vec_of_actions(v: &[Action]) -> Tensor {
    let v: Vec<Tensor> = v.into_iter().map(|action| action.encode()).collect();
    Tensor::stack(&v, 0)
}

#[cfg(test)]
mod tests {
    use crate::model::utils::encode_batch;

    use super::*;

    #[test]
    fn test_encode() {
        let enc_action = encode_batch(&[
            Action::ToFood.encode(),
            Action::ToWater.encode(),
            Action::Stationary.encode(),
            Action::ToWater.encode(),
        ]);

        print!("{}", enc_action);

        assert_eq!(enc_action.size(), vec![4, 4]);
    }
}

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        match rng.gen_range(0..=2) {
            0 => Action::ToFood,
            1 => Action::ToWater,
            _ => Action::Stationary,
        }
    }
}
