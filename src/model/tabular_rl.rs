use itertools::Itertools;
use krabmaga::{engine::state::State, HashMap};
use rand::rngs::StdRng;
use std::fmt::Debug;
use std::marker::PhantomData;
use strum::IntoEnumIterator;
use tuple_conv::RepeatedTuple;

use crate::config::core_config;

use super::{
    agent_state::DiscrRep,
    board::Board,
    history::{History, SAR},
    q_table::{self, QTable},
};

pub struct SARSAModel<T, S, L, A>
where
    T: DiscrRep<S, L> + Clone,
    S: std::cmp::Eq + std::hash::Hash + Clone,
    L: std::cmp::Eq + std::hash::Hash + Clone,
    A: std::cmp::Eq + std::hash::Hash + Clone,
{
    pub q_tbls: HashMap<u32, QTable<S, L, A>>,
    agent_state_type: PhantomData<T>,
}

impl<T, S, L, A> SARSAModel<T, S, L, A>
where
    T: DiscrRep<S, L> + Clone,
    S: std::cmp::Eq + std::hash::Hash + Clone + Debug,
    L: std::cmp::Eq + std::hash::Hash + Clone + Debug,
    A: std::cmp::Eq + std::hash::Hash + Clone + Debug + IntoEnumIterator,
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
        SARSAModel {
            q_tbls,
            agent_state_type: PhantomData,
        }
    }

    pub fn step(
        &mut self,
        t: i32,
        agent_hist: &HashMap<u32, History<T, S, L, A>>,
    ) -> Option<HashMap<u32, SAR<T, S, L, A>>> {
        let tau_: i32 = t - core_config().rl.SARSA_N as i32 - 1;

        // do update
        if tau_ >= 0 {
            // update all agents in turn
            for (id, hist) in agent_hist.iter() {
                let tab = self.get_table_by_id_mut(*id);
                let traj = &hist.trajectory;

                let tau = tau_ as usize;
                let n = core_config().rl.SARSA_N as usize;
                let mut g: f32 = 0.0;

                // sum n rewards (discounted back)
                for i in (tau + 1)..=(tau + n) {
                    // assuming index (s0,a0,r1),(s1,a1,r2)...
                    // book assumes (s0,a0),(s1,a1,r1)...
                    let r_i = traj[i - 1].reward.val;
                    g += core_config().rl.GAMMA.powf((i - tau - 1) as f32) * r_i as f32;
                }

                // bootstrap using q(n+1)
                let q_btstrap = tab
                    .get(&traj[tau + n].representation())
                    .expect("all possible state-actions will be in the QTable");
                g += core_config().rl.GAMMA.powf(n as f32) * q_btstrap;

                // update q for (s_tau,a_tau)
                let mut q_tau = *tab
                    .get(&traj[tau].representation())
                    .expect("all possible state-actions will be in the QTable");
                q_tau += core_config().rl.ALPHA * (g - q_tau);
                let old_q = tab.insert(traj[tau].representation(), q_tau);
                // println!("{:?} -> {:?}", old_q, q_tau)
            }
        }
        let mut last_sars = HashMap::new();
        if t > 0 {
            for (id, hist) in agent_hist.iter() {
                last_sars.insert(*id, hist.trajectory.last().unwrap().clone());
            }
            return Some(last_sars);
        }
        None
    }

    pub fn get_table_by_id_mut(&mut self, id: u32) -> &mut HashMap<(Vec<(S, L)>, A), f32> {
        self.q_tbls
            .get_mut(&id)
            .expect("qtable was initialised for all agent id's")
            .get_tab_mut()
    }

    pub fn get_table_by_id(&self, id: u32) -> &HashMap<(Vec<(S, L)>, A), f32> {
        self.q_tbls
            .get(&id)
            .expect("qtable was initialised for all agent id's")
            .get_tab()
    }

    pub fn sample_action_by_id(&self, id: u32, state: &Vec<(S, L)>, rng: &mut StdRng) -> A {
        let (a, q_optimal) = self
            .q_tbls
            .get(&id)
            .expect("qtable was initialised for all agent id's")
            .sample_action(state, rng);
        if id == 0 {
            // println!("{}", q_optimal)
        }
        a
    }
}
