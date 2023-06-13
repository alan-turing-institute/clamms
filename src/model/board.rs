use super::environment::Resource;
use super::history::History;
use super::{environment::EnvItem, forager::Forager};
use crate::config::{INIT_FOOD, INIT_WATER};
use krabmaga::HashMap;
// use hashbrown::HashMap;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::{
    fields::sparse_object_grid_2d::SparseGrid2D, location::Int2D, state::State,
};
use rand::Rng;
use std::hash::{Hash, Hasher};

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

pub struct Board {
    pub step: u64,
    pub resource_grid: DenseGrid2D<Patch>,
    pub agent_grid: SparseGrid2D<Forager>,
    pub dim: (u16, u16),
    pub num_agents: u8,
    pub agent_histories: HashMap<u32, History>,
    pub resource_locations: HashMap<Resource, Vec<Int2D>>,
}

impl Board {
    pub fn new(dim: (u16, u16), num_agents: u8) -> Board {
        Board {
            step: 0,
            agent_grid: SparseGrid2D::new(dim.0.into(), dim.0.into()),
            resource_grid: DenseGrid2D::new(dim.0.into(), dim.1.into()),
            dim,
            num_agents,
            agent_histories: HashMap::new(),
            resource_locations: HashMap::new(),
        }
    }
}

impl State for Board {
    fn init(&mut self, schedule: &mut krabmaga::engine::schedule::Schedule) {
        self.step = 0;
        let mut rng = rand::thread_rng();

        for n in 0..self.num_agents {
            let x: u16 = rng.gen_range(1..self.dim.0);
            let y: u16 = rng.gen_range(1..self.dim.1);

            let id: u32 = n.into();

            let agent = Forager::new(
                id,
                Int2D {
                    x: x.into(),
                    y: y.into(),
                },
                INIT_FOOD,
                INIT_WATER,
            );

            // Init empty history
            self.agent_histories.insert(id, History::new());

            // Put the agent in your state
            schedule.schedule_repeating(Box::new(agent), 0., 0);
        }

        let mut id = 0;
        for i in 0..self.dim.0 {
            for j in 0..self.dim.1 {
                let pos = Int2D {
                    x: i.into(),
                    y: j.into(),
                };
                let item: EnvItem = rand::random();
                let patch = Patch::new(id, item);
                self.resource_grid.set_object_location(patch, &pos);
                if let EnvItem::Resource(resource) = patch.env_item {
                    let v = self.resource_locations.get_mut(&resource).expect("Resource HashMaps initialised in init");
                    v.push(pos.to_owned());
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
        self.agent_grid = SparseGrid2D::new(self.dim.0.into(), self.dim.1.into());
    }
}
