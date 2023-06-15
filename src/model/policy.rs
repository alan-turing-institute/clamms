use krabmaga::engine::state::State;

use super::{action::Action, agent_state::AgentState};

pub trait Policy {
    fn choose_action(&self, agent_state: &AgentState) -> Action;
}
