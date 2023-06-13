#[derive(Debug, Clone, PartialEq)]
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
}
