use super::{agent_state::DiscrRep, history::History, q_table::QTable};
use crate::config::core_config;
use itertools::Itertools;
use krabmaga::HashMap;
use rand::rngs::StdRng;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::marker::PhantomData;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct SARSAModel<T, S, L, A>
where
    T: DiscrRep<S, L> + Clone,
    S: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize,
    L: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize,
    A: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize + IntoEnumIterator,
{
    /// Q tables indexed by agent ID.
    pub q_tbls: HashMap<u32, QTable<S, L, A>>,
    /// Only learn single table if value is false, while one per agent if true.
    multi_policy: bool,
    agent_state_type: PhantomData<T>,
    pub checkpoint_itr: Option<i32>,
}

impl<T, S, L, A> SARSAModel<T, S, L, A>
where
    T: DiscrRep<S, L> + Clone,
    S: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize,
    L: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize,
    A: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize + IntoEnumIterator,
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
            checkpoint_itr: None,
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

    pub fn save(self) {
        let mut total_itr = core_config().world.N_STEPS;
        if core_config().rl.LOAD_MODEL {
            total_itr += self.checkpoint_itr.expect("set when model loaded");
        }
        let mut f = File::create(format!(
            "multiP_{}__agents_{}__trading_{}__totalItr_{}.json",
            if core_config().rl.MULTI_POLICY { 1 } else { 0 },
            core_config().world.N_AGENTS,
            if core_config().world.HAS_TRADING {
                1
            } else {
                0
            },
            total_itr
        ))
        .unwrap();

        writeln!(
            f,
            "{}",
            serde_json::to_string_pretty(&hashmap_keys_to_string(self.q_tbls)).unwrap()
        )
        .unwrap();
    }
}

fn hashmap_keys_to_string<S, L, A>(
    mut m: HashMap<u32, QTable<S, L, A>>,
) -> HashMap<String, HashMap<String, f32>>
where
    S: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize,
    L: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize,
    A: std::cmp::Eq + std::hash::Hash + Clone + Debug + IntoEnumIterator + Serialize,
{
    let mut n = HashMap::new();
    if core_config().rl.MULTI_POLICY {
        for (k, v) in m.into_iter() {
            let mut nn = HashMap::new();
            for (kk, vv) in v.tab {
                let mut s = String::new();
                s += &(kk.0.iter().map(|x| format!("{:?}", x)).join("") + &format!("{:?}", kk.1));
                nn.insert(s, vv);
            }
            n.insert(k.to_string(), nn);
        }
    } else {
        let mut nn = HashMap::new();
        for (kk, vv) in &mut m.remove(&0).unwrap().tab {
            let mut s = String::new();
            s += &(kk.0.iter().map(|x| format!("{:?}", x)).join("") + &format!("{:?}", kk.1));
            nn.insert(s, *vv);
        }
        n.insert(String::from("0"), nn);
    }

    n
}
