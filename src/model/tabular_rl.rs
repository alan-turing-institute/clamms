use krabmaga::HashMap;
pub struct SARSAModel<S, A>
where
    S: std::cmp::Eq + std::hash::Hash + Clone,
    A: std::cmp::Eq + std::hash::Hash + Clone,
{
    pub q_tbl: HashMap<(S, A), f32>,
}

impl<S, A> SARSAModel<S, A>
where
    S: std::cmp::Eq + std::hash::Hash + Clone,
    A: std::cmp::Eq + std::hash::Hash + Clone,
{
    pub fn new(states: &Vec<S>, actions: &Vec<A>) -> Self {
        let mut q_tbl = HashMap::new();
        for s in states {
            for a in actions {
                q_tbl.insert((s.to_owned(), a.to_owned()), 0.0);
            }
        }
        SARSAModel { q_tbl }
    }
}
