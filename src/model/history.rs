use super::{
    action::Action,
    agent_state::{AgentState, AgentStateItems, DiscrRep, InvLevel},
    board::{AgentOffer, ClammsInt2D},
    reward::Reward,
    trader::Offer,
};
use itertools::izip;
use krabmaga::engine::location::Int2D;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct History<T, S, L, A>
where
    T: DiscrRep<S, L> + Clone,
    A: Clone,
    S: Clone,
    L: Clone,
{
    pub trajectory: Vec<SAR<T, S, L, A>>,
    pub times: Vec<u64>,
    pub positions: Vec<ClammsInt2D>,
    pub trades: Vec<Option<AgentOffer>>,
    agent_state_items: PhantomData<S>,
    agent_state_item_levels: PhantomData<L>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SAR<T, S, L, A>
where
    T: DiscrRep<S, L>,
    A: Clone,
    S: Clone,
    L: Clone,
{
    #[serde(rename(serialize = "S"))]
    pub state: T,
    #[serde(rename(serialize = "A"))]
    pub action: A,
    #[serde(rename(serialize = "R"))]
    pub reward: Reward,
    // TODO: check why needed
    #[serde(skip_serializing)]
    agent_state_items: PhantomData<S>,
    #[serde(skip_serializing)]
    agent_state_item_levels: PhantomData<L>,
}

impl<T, S, L, A> History<T, S, L, A>
where
    T: DiscrRep<S, L> + Clone,
    A: Clone,
    S: Clone,
    L: Clone,
{
    pub fn new() -> Self {
        Self {
            trajectory: Vec::new(),
            times: Vec::new(),
            positions: Vec::new(),
            trades: Vec::new(),
            agent_state_items: PhantomData,
            agent_state_item_levels: PhantomData,
        }
    }
    pub fn push(&mut self, sar: SAR<T, S, L, A>, time: u64, pos: Int2D, trade: Option<AgentOffer>) {
        self.trajectory.push(sar);
        self.times.push(time);
        self.positions.push(pos.into());
        self.trades.push(trade);
    }
    pub fn last_state_action(&self) -> Option<(T, A)> {
        let len = self.trajectory.len();
        if len > 0 {
            Some((
                self.trajectory[self.trajectory.len() - 1].state.clone(),
                self.trajectory[self.trajectory.len() - 1].action.clone(),
            ))
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        self.trajectory.len()
    }
    pub fn to_skipped_history(&self, step_size: usize) -> Self {
        izip!(&self.trajectory, &self.times, &self.positions, &self.trades)
            .step_by(step_size)
            .fold(History::new(), |mut acc, (state, time, pos, trade)| {
                acc.trajectory.push(state.clone());
                acc.times.push(*time);
                acc.positions.push(*pos);
                acc.trades.push(*trade);
                acc
            })
    }
}

impl<T, S, L, A> SAR<T, S, L, A>
where
    T: DiscrRep<S, L> + Clone,
    A: Clone,
    S: Clone,
    L: Clone,
{
    pub fn new(state: T, action: A, reward: Reward) -> Self {
        SAR {
            state,
            action,
            reward,
            agent_state_items: PhantomData,
            agent_state_item_levels: PhantomData,
        }
    }

    pub fn representation(&self) -> (Vec<(S, L)>, A) {
        (self.state.representation(), self.action.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_history() -> History<AgentState, AgentStateItems, InvLevel, Action> {
        History {
            trajectory: vec![SAR::new(
                AgentState {
                    food: 0,
                    water: 0,
                    min_steps_to_food: None,
                    min_steps_to_water: None,
                    min_steps_to_trader: None, // last_action: None,
                },
                Action::Stationary,
                Reward { val: -1 },
            )],
            positions: vec![ClammsInt2D::new((0, 0))],
            times: vec![0],
            trades: vec![None],
            agent_state_items: PhantomData,
            agent_state_item_levels: PhantomData,
        }
    }

    #[test]
    fn test_history_push() {
        let mut history = get_test_history();
        let sar = SAR::new(
            AgentState {
                food: 0,
                water: 0,
                min_steps_to_food: None,
                min_steps_to_water: None,
                min_steps_to_trader: None, // last_action: None,
            },
            Action::Stationary,
            Reward { val: -1 },
        );
        let sar2 = SAR::new(
            AgentState {
                food: 0,
                water: 0,
                min_steps_to_food: None,
                min_steps_to_water: None,
                min_steps_to_trader: None, // last_action: None,
            },
            Action::Stationary,
            Reward { val: -2 },
        );
        history.push(sar.clone(), 1, Int2D { x: 1, y: 0 }, None);

        assert_eq!(history.len(), 2);
        // Cannot use matches! on struct RHS?
        // assert!(matches!(history.trajectory.last().unwrap(), sar)));
        assert_eq!(history.trajectory.last().unwrap(), &sar);
        assert_ne!(history.trajectory.last().unwrap(), &sar2);
    }

    // #[test]
    // fn test_last_state_action() {
    //     assert!(matches!(
    //         get_test_history().last_state_action(),
    //         Some((
    //             AgentState {
    //                 food: 0,
    //                 water: 0,
    //                 // last_action: None,
    //             },
    //             Action::Stationary
    //         ))
    //     ))
    // }
}
