use super::serde_utils;
use crate::config::core_config;
use itertools::Itertools;
use krabmaga::HashMap;
use rand::{rngs::StdRng, Rng};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum::IntoEnumIterator;
use tuple_conv::RepeatedTuple;

#[derive(Debug, Serialize, Deserialize)]
pub struct QTable<S, L, A>
where
    S: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize + DeserializeOwned,
    L: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize + DeserializeOwned,
    A: std::cmp::Eq
        + std::hash::Hash
        + Clone
        + Debug
        + IntoEnumIterator
        + Serialize
        + DeserializeOwned,
{
    #[serde(with = "serde_utils")]
    pub tab: HashMap<QKey<S, L, A>, f32>,
    pub action_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QKey<S, L, A>(pub Vec<(S, L)>, pub A);

impl<S, L, A> QKey<S, L, A> {
    pub fn from_tuple(tup: (Vec<(S, L)>, A)) -> QKey<S, L, A> {
        QKey(tup.0, tup.1)
    }
}

impl<S, L, A> QTable<S, L, A>
where
    S: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize + DeserializeOwned,
    L: std::cmp::Eq + std::hash::Hash + Clone + Debug + Serialize + DeserializeOwned,
    A: std::cmp::Eq
        + std::hash::Hash
        + Clone
        + Debug
        + Serialize
        + IntoEnumIterator
        + DeserializeOwned,
{
    pub fn new(state_items: Vec<S>, state_levels: Vec<L>, actions: Vec<A>) -> Self {
        let mut q_tbl = HashMap::new();
        let mut combs_for_all_state_items = Vec::new();
        for s in state_items {
            let mut levels_for_item = Vec::new();
            for l in state_levels.clone() {
                levels_for_item.push((s.clone(), l))
            }
            combs_for_all_state_items.push(levels_for_item);
        }

        let combs = combs_for_all_state_items
            .clone()
            .into_iter()
            .multi_cartesian_product()
            .collect_vec();

        // println!("{:?}", combs);

        // let manual = combs_for_all_state_items[0]
        //     .clone()
        //     .into_iter()
        //     .cartesian_product(combs_for_all_state_items[1].clone())
        //     .into_iter()
        //     .collect_vec();

        let q = combs
            .into_iter()
            .cartesian_product(actions)
            .into_iter()
            .collect_vec();

        for el in q {
            // println!("{:?}", el);
            let q_key = QKey(el.0, el.1);
            q_tbl.insert(q_key, core_config().rl.INIT_Q_VALUES);
        }

        QTable {
            tab: q_tbl,
            action_count: A::iter().count(),
        }
    }

    pub fn get_tab_mut(&mut self) -> &mut HashMap<QKey<S, L, A>, f32> {
        &mut self.tab
    }
    pub fn get_tab(&self) -> &HashMap<QKey<S, L, A>, f32> {
        &self.tab
    }

    pub fn sample_action(&self, state: &Vec<(S, L)>, rng: &mut StdRng) -> (A, f32) {
        // Pick first action and evaluate q
        let mut action_itr = A::iter();
        let mut optimal_a = action_itr.next().unwrap();
        let mut q_optimal = self
            .get_tab()
            .get(&QKey(state.to_owned(), optimal_a.clone()))
            .unwrap();

        // Continue iterating through the next actions, to find optimal action
        for a in action_itr {
            let q_a = self
                .get_tab()
                .get(&QKey(state.to_owned(), a.clone()))
                .unwrap();
            if q_a > q_optimal {
                optimal_a = a;
                q_optimal = q_a
            }
        }
        let r: f32 = rng.gen();
        // if exploring, pick randomly
        if r < core_config().rl.EPSILON {
            optimal_a = self.pick_rnd(rng);
        }
        (optimal_a, *q_optimal)
    }
    fn pick_rnd(&self, rng: &mut StdRng) -> A {
        let a_idx: usize = rng.gen_range(0..self.action_count);
        A::iter().nth(a_idx).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::agent_state::InvLevel;

    #[test]
    fn test_multi_product() {
        let combs = vec![
            vec![
                InvLevel::Critical,
                InvLevel::Low,
                InvLevel::Medium,
                InvLevel::High,
            ];
            3
        ]
        .clone()
        .into_iter()
        .multi_cartesian_product()
        .collect_vec();
        // Should be: 4 ** 3 with each position taking all possible variants of the enum
        assert_eq!(combs.len(), 64)
    }
}
