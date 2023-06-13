use super::{action::Action, agent_state::AgentState};

pub trait Policy {
    fn chose_action(&self, agent_state: &AgentState) -> Action;
}
