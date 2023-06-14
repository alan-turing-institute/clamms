use std::fmt::Debug;

use itertools::Itertools;
use krabmaga::{engine::state::State, HashMap};
use tuple_conv::RepeatedTuple;

use crate::config::core_config;

use super::{
    board::Board,
    history::History,
    q_table::{self, QTable},
};
pub struct SARSAModel<S, L, A>
where
    S: std::cmp::Eq + std::hash::Hash + Clone,
    L: std::cmp::Eq + std::hash::Hash + Clone,
    A: std::cmp::Eq + std::hash::Hash + Clone,
{
    pub q_tbls: HashMap<u32, QTable<S, L, A>>,
}

impl<S, L, A> SARSAModel<S, L, A>
where
    S: std::cmp::Eq + std::hash::Hash + Clone + Debug,
    L: std::cmp::Eq + std::hash::Hash + Clone + Debug,
    A: std::cmp::Eq + std::hash::Hash + Clone + Debug,
{
    // Vec< Vec<dim=num levels for each resource> dim=num different resources>
    pub fn new(
        agent_ids: Vec<u32>,
        state_items: Vec<S>,
        state_levels: Vec<L>,
        actions: Vec<A>,
    ) -> Self {
        let mut q_tbls = HashMap::new();
        for id in agent_ids {
            q_tbls.insert(
                id,
                QTable::new(state_items.clone(), state_levels.clone(), actions.clone()),
            );
        }
        SARSAModel { q_tbls }
    }

    pub fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any_mut().downcast_mut::<Board>().unwrap();
        for (id, hist) in state.agent_histories.iter() {
            let tab = self
                .q_tbls
                .get(id)
                .expect("qtable was initialised for all agent id's");
        }
    }
}
