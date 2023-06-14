use std::fmt::Debug;

use itertools::Itertools;
use krabmaga::HashMap;
use tuple_conv::RepeatedTuple;

use crate::config::core_config;
pub struct SARSAModel<S, L, A>
where
    S: std::cmp::Eq + std::hash::Hash + Clone,
    L: std::cmp::Eq + std::hash::Hash + Clone,
    A: std::cmp::Eq + std::hash::Hash + Clone,
{
    pub q_tbl: HashMap<(((S, L), (S, L)), A), f32>,
}

impl<S, L, A> SARSAModel<S, L, A>
where
    S: std::cmp::Eq + std::hash::Hash + Clone + Debug,
    L: std::cmp::Eq + std::hash::Hash + Clone + Debug,
    A: std::cmp::Eq + std::hash::Hash + Clone + Debug,
{
    // Vec< Vec<dim=num levels for each resource> dim=num different resources>
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

        // let product: Vec<Vec<(S, L)>> = combs_for_all_state_items
        //     .remove(0)
        //     .clone()
        //     .into_iter()
        //     .cartesian_product(combs_for_all_state_items.remove(0))
        //     .into_iter()
        //     .map(|t| t.to_vec())
        //     .collect();

        // for i in 0..combs_for_all_state_items.len() - 2 {
        //     for p in product {
        //         p.iter()
        //             .cartesian_product(combs_for_all_state_items[i])
        //             .into_iter()
        //             .map(|t| t.to_vec())
        //             .collect()
        //     }
        // }

        let manual = combs_for_all_state_items[0]
            .clone()
            .into_iter()
            .cartesian_product(combs_for_all_state_items[1].clone())
            .into_iter()
            .collect_vec();

        let q = manual
            .into_iter()
            .cartesian_product(actions)
            .into_iter()
            .collect_vec();

        for el in q {
            println!("{:?}", el);
            q_tbl.insert(el, core_config().rl.INIT_Q_VALUES);
        }

        SARSAModel { q_tbl }
    }
}
