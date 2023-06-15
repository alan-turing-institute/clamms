use crate::config::{core_config, CLAMMS_CONFIG};

use super::action::Action;
use super::agent_state::{AgentState, AgentStateItems, InvLevel};
use super::environment::Resource;
use super::history::History;
use super::tabular_rl::SARSAModel;
use super::{environment::EnvItem, forager::Forager};
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::{
    fields::sparse_object_grid_2d::SparseGrid2D, location::Int2D, state::State,
};
use krabmaga::hashbrown::HashSet;
use krabmaga::HashMap;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct Patch {
    pub id: u32,
    pub env_item: EnvItem,
}

impl Patch {
    pub fn new(id: u32, env_item: EnvItem) -> Self {
        Patch { id, env_item }
    }
}

impl Hash for Patch {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Patch {}

impl PartialEq for Patch {
    fn eq(&self, other: &Patch) -> bool {
        self.id == other.id
    }
}

// impl fmt::Display for Patch {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{} value {}", self.id, self.env_item)
//     }
// }

/// Clamms version of Int2D with trait implementations and conversion to Int2D
#[derive(Clone, Copy, Serialize, Deserialize, Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct ClammsInt2D {
    pub x: i32,
    pub y: i32,
}

impl From<ClammsInt2D> for Int2D {
    fn from(value: ClammsInt2D) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
///
pub fn read_resource_locations(input: &str) -> BTreeMap<Resource, Vec<Int2D>> {
    serde_json::from_str::<BTreeMap<Resource, Vec<ClammsInt2D>>>(input)
        .unwrap()
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().map(Int2D::from).collect()))
        .collect()
}

pub fn example_board(dim: (u16, u16)) -> BTreeMap<Resource, Vec<ClammsInt2D>> {
    let mut map = BTreeMap::new();
    map.insert(Resource::Food, vec![]);
    map.insert(Resource::Water, vec![]);
    for i in 1..7 {
        for j in 1..7 {
            let v = map.get_mut(&Resource::Food).unwrap();
            v.push(ClammsInt2D { x: i, y: j });
        }
    }
    for i in (dim.0 / 2 - 2)..=(dim.0 / 2 + 2) {
        for j in 1..dim.1 {
            let v = map.get_mut(&Resource::Water).unwrap();
            v.push(ClammsInt2D {
                x: i.into(),
                y: j.into(),
            });
        }
    }
    map
}

pub struct Board {
    pub step: u64,
    pub resource_grid: DenseGrid2D<Patch>,
    pub agent_grid: DenseGrid2D<Forager>,
    pub dim: (u16, u16),
    pub num_agents: u8,
    pub agent_histories: HashMap<u32, History<AgentState, AgentStateItems, InvLevel, Action>>,
    // TODO: consider refactor to BTreeMap if issues occur around deterministic iteration
    pub resource_locations: BTreeMap<Resource, Vec<Int2D>>,
    pub rng: StdRng,
    pub model: SARSAModel<AgentState, AgentStateItems, InvLevel, Action>,
    pub loaded_map: bool,
}

impl Board {
    pub fn new(
        dim: (u16, u16),
        num_agents: u8,
        model: SARSAModel<AgentState, AgentStateItems, InvLevel, Action>,
    ) -> Board {
        Board {
            step: 0,
            agent_grid: DenseGrid2D::new(dim.0.into(), dim.0.into()),
            resource_grid: DenseGrid2D::new(dim.0.into(), dim.1.into()),
            dim,
            num_agents,
            agent_histories: HashMap::new(),
            resource_locations: BTreeMap::new(),
            rng: StdRng::from_entropy(),
            model,
            loaded_map: false,
        }
    }
    pub fn new_with_seed(
        dim: (u16, u16),
        num_agents: u8,
        seed: u64,
        model: SARSAModel<AgentState, AgentStateItems, InvLevel, Action>,
    ) -> Board {
        Board {
            step: 0,
            agent_grid: DenseGrid2D::new(dim.0.into(), dim.0.into()),
            resource_grid: DenseGrid2D::new(dim.0.into(), dim.1.into()),
            dim,
            num_agents,
            agent_histories: HashMap::new(),
            resource_locations: BTreeMap::new(),
            rng: StdRng::seed_from_u64(seed),
            model,
            loaded_map: false,
        }
    }
    pub fn new_with_seed_resources(
        dim: (u16, u16),
        num_agents: u8,
        seed: u64,
        map_locations: &str,
        model: SARSAModel<AgentState, AgentStateItems, InvLevel, Action>,
    ) -> Board {
        let path =
            std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(map_locations);
        let resource_locations = read_resource_locations(&std::fs::read_to_string(path).unwrap());
        Board {
            step: 0,
            agent_grid: DenseGrid2D::new(dim.0.into(), dim.0.into()),
            resource_grid: DenseGrid2D::new(dim.0.into(), dim.1.into()),
            dim,
            num_agents,
            agent_histories: HashMap::new(),
            resource_locations,
            rng: StdRng::seed_from_u64(seed),
            loaded_map: true,
            model,
        }
    }
}

impl State for Board {
    fn init(&mut self, schedule: &mut krabmaga::engine::schedule::Schedule) {
        self.step = 0;
        for n in 0..self.num_agents {
            let x: u16 = self.rng.gen_range(1..self.dim.0);
            let y: u16 = self.rng.gen_range(1..self.dim.1);

            let id: u32 = n.into();

            let agent = Forager::new(
                id,
                Int2D {
                    x: x.into(),
                    y: y.into(),
                },
                core_config().agent.INIT_FOOD,
                core_config().agent.INIT_WATER,
            );

            // Init empty history
            self.agent_histories.insert(id, History::new());

            // Put the agent in your state
            schedule.schedule_repeating(Box::new(agent), 0., 0);
        }

        let resource_lookup = if !self.loaded_map {
            // Init empty resource locations
            for resource in Resource::iter() {
                self.resource_locations.insert(resource, Vec::new());
            }
            None
        } else {
            let mut resource_lookup: HashMap<Int2D, Resource> = HashMap::new();
            self.resource_locations.iter().for_each(|(&res, v)| {
                for loc in v.iter() {
                    resource_lookup.insert(*loc, res);
                }
            });
            Some(resource_lookup)
        };

        let mut id = 0;
        for i in 0..self.dim.0 {
            for j in 0..self.dim.1 {
                let pos = Int2D {
                    x: i.into(),
                    y: j.into(),
                };
                let item = if let Some(resource_lookup) = resource_lookup.as_ref() {
                    if let Some(resource) = resource_lookup.get(&pos) {
                        EnvItem::Resource(*resource)
                    } else {
                        EnvItem::Land
                    }
                } else {
                    self.rng.gen()
                };

                let patch = Patch::new(id, item);
                self.resource_grid.set_object_location(patch, &pos);
                if !self.loaded_map {
                    if let EnvItem::Resource(resource) = patch.env_item {
                        let v = self
                            .resource_locations
                            .get_mut(&resource)
                            .expect("HashMap initialised for all resource types");
                        v.push(pos.to_owned());
                    }
                }
                id += 1;
            }
        }
    }

    fn after_step(&mut self, schedule: &mut krabmaga::engine::schedule::Schedule) {
        self.step += 1
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn update(&mut self, step: u64) {
        // lazy_update stops the field being searchable!
        self.resource_grid.update();
        self.agent_grid.lazy_update();
    }

    fn reset(&mut self) {
        self.step = 0;
        self.resource_grid = DenseGrid2D::new(self.dim.0.into(), self.dim.1.into());
        self.agent_grid = DenseGrid2D::new(self.dim.0.into(), self.dim.1.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_LOCATIONS: &str = r#"{
        "Food": [
          {"x": 19,"y": 19},
          {"x": 19,"y": 18},
          {"x": 18, "y": 19},
          {"x": 18,"y": 18}
        ],
        "Water": [
          {"x": 3,"y": 3},
          {"x": 3,"y": 2},
          {"x": 2,"y": 3},
          {"x": 2,"y": 2}
        ]
      }"#;
    #[test]
    fn test_read_resources() {
        let _ = read_resource_locations(TEST_LOCATIONS);
    }
    #[test]
    fn test_example_board() {
        println!(
            "{}",
            serde_json::to_string(&example_board((42, 42))).unwrap()
        );
    }
}
