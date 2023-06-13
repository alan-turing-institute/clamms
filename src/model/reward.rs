#[derive(Debug, Clone)]
pub struct Reward {
    pub val: i32,
}

impl Reward {
    pub fn new(val: i32) -> Self {
        Reward { val }
    }
    pub fn from_inv_linear(inv_count: i32) -> Self {
        Reward::new(inv_count)
    }
}
