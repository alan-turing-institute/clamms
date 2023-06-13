use super::{action::Action, agent_state::AgentState, reward::Reward};

#[derive(Clone, Debug)]
pub struct History {
    trajectory: Vec<SAR>,
}

#[derive(Clone, Debug, PartialEq)]
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
    pub fn last_action(&self) -> Option<Action> {
        let len = self.trajectory.len();
        if len > 0 {
            Some(self.trajectory[self.trajectory.len() - 1].action.clone())
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        self.trajectory.len()
    }
}

impl SAR {
    pub fn new(state: AgentState, action: Action, reward: Reward) -> Self {
        SAR {
            state,
            action,
            reward,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_history() -> History {
        History {
            trajectory: vec![SAR::new(
                AgentState {
                    food: 0,
                    water: 0,
                    food_dist: 5,
                    water_dist: 10,
                    last_action: None,
                },
                Action::Stationary,
                Reward { val: -1 },
            )],
        }
    }

    #[test]
    fn test_history_push() {
        let mut history = get_test_history();
        let sar = SAR::new(
            AgentState {
                food: 0,
                water: 0,
                food_dist: 5,
                water_dist: 10,
                last_action: None,
            },
            Action::Stationary,
            Reward { val: -1 },
        );
        let sar2 = SAR::new(
            AgentState {
                food: 0,
                water: 0,
                food_dist: 5,
                water_dist: 10,
                last_action: None,
            },
            Action::Stationary,
            Reward { val: -2 },
        );
        history.push(sar.clone());

        assert_eq!(history.len(), 2);
        // Cannot use matches! on struct RHS?
        // assert!(matches!(history.trajectory.last().unwrap(), sar)));
        assert_eq!(history.trajectory.last().unwrap(), &sar);
        assert_ne!(history.trajectory.last().unwrap(), &sar2);
    }

    #[test]
    fn test_last_action() {
        assert!(matches!(
            get_test_history().last_action(),
            Some(Action::Stationary)
        ))
    }
}
