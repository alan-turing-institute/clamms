use krabmaga::engine::state::State;

use super::{action::Action, agent_state::AgentState};

pub trait Policy {
    fn chose_action(&self, state: &mut dyn State, agent_state: &AgentState) -> Action;
}
