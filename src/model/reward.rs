use serde::{Deserialize, Serialize};
use tch::Tensor;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reward {
    pub val: i32,
}

impl Reward {
    pub fn new(val: i32) -> Self {
        Reward { val }
    }
    /// Recieve a negative reward if resource counts are 0 or below negative reward is linearly
    /// proportional to count
    pub fn from_inv_count_linear(food_count: i32, water_count: i32) -> Self {
        let food_reward = 0.min(food_count);
        let water_reward = 0.min(water_count);
        Reward::new(food_reward + water_reward)
    }
    /// Function to encode `Reward` as a `tch` `Tensor`.
    pub fn encode(&self) -> Tensor {
        Tensor::from_slice(&[self.val]).internal_cast_float(true)
    }
}


#[cfg(test)]
mod tests {
    use crate::model::utils::encode_batch;

    use super::*;

    #[test]
    fn test_encode() {
        let r1 = Reward::new(12);
        let r2 = Reward::new(34);
        
        let enc_reward = encode_batch(&[r1.encode(), r2.encode()]);

        assert_eq!(enc_reward.size(), vec![2, 1]);
    }
     
}