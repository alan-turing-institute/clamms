use krabmaga::engine::state::State;

use super::{action::Action, agent_state::AgentState};

pub trait Policy {
    fn chose_action(&self, agent_state: &AgentState) -> Action;
    fn choose_action(&self, state: &dyn State) -> Action;
}
