use super::{agent_state::DiscrRep, history::History, q_table::QTable};
use crate::config::core_config;
use krabmaga::HashMap;
use rand::rngs::StdRng;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct SARSAModel<T, S, L, A>
where
    T: DiscrRep<S, L> + Clone,
    S: std::cmp::Eq + std::hash::Hash + Clone,
    L: std::cmp::Eq + std::hash::Hash + Clone,
    A: std::cmp::Eq + std::hash::Hash + Clone,
{
    /// Q tables indexed by agent ID.
    pub q_tbls: HashMap<u32, QTable<S, L, A>>,
    /// Only learn single table if value is false, while one per agent if true.
    multi_policy: bool,
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
        multi_policy: bool,
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
            multi_policy,
            agent_state_type: PhantomData,
        }
    }

    fn policy_id(&self, id: u32) -> u32 {
        if self.multi_policy {
            id
        } else {
            0
        }
    }

    pub fn step(&mut self, t: i32, agent_hist: &BTreeMap<u32, History<T, S, L, A>>) {
        let tau_: i32 = t - core_config().rl.SARSA_N as i32 - 1;

        // do update
        if tau_ >= 0 {
            // update all agents in turn
            for (id, hist) in agent_hist.iter() {
                let tab = self.get_table_by_id_mut(self.policy_id(*id));
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
    }

    pub fn get_table_by_id_mut(&mut self, id: u32) -> &mut HashMap<(Vec<(S, L)>, A), f32> {
        self.q_tbls
            .get_mut(&self.policy_id(id))
            .expect("qtable was initialised for all agent id's")
            .get_tab_mut()
    }

    pub fn get_table_by_id(&self, id: u32) -> &HashMap<(Vec<(S, L)>, A), f32> {
        self.q_tbls
            .get(&self.policy_id(id))
            .expect("qtable was initialised for all agent id's")
            .get_tab()
    }

    pub fn sample_action_by_id(&self, id: u32, state: &Vec<(S, L)>, rng: &mut StdRng) -> A {
        let (a, q_optimal) = self
            .q_tbls
            .get(&self.policy_id(id))
            .expect("qtable was initialised for all agent id's")
            .sample_action(state, rng);
        if id == 0 {
            // println!("{}", q_optimal)
        }
        a
    }
}
