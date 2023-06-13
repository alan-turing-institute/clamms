use super::action::Action;

#[derive(Debug, Clone, PartialEq)]
pub struct AgentState {
    pub food: i32,
    pub water: i32,
    pub food_dist: u32,
    pub water_dist: u32,
    pub last_action: Option<Action>,
}
