use super::{action::Action, agent_state::AgentState, reward::Reward};

#[derive(Clone, Debug)]
pub struct History {
    trajectory: Vec<SAR>,
}

#[derive(Clone, Debug)]
pub struct SAR {
    pub state: AgentState,
    pub action: Action,
    pub reward: Reward,
}

impl Default for History {
    fn default() -> Self {
        Self {
            trajectory: Vec::new(),
        }
    }
}

impl History {
    pub fn new() -> Self {
        Self {
            trajectory: Vec::new(),
        }
    }
    pub fn push(&mut self, sar: SAR) {
        self.trajectory.push(sar);
    }
}
