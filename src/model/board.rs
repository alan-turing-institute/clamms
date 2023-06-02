use krabmaga::{engine::{state::State,fields::{sparse_object_grid_2d::SparseGrid2D}, location::Int2D}};
use rand::{Rng};
use super::{walker::Walker, env_item::EnvItem,};
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use std::hash::{Hash, Hasher};
use core::fmt;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct Patch {
    pub id: u32,
    pub env_item: EnvItem,
}

impl Patch {
    pub fn new(id: u32, env_item: EnvItem) -> Self {
        Patch {
            id,
            env_item,
        }
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
    pub field: DenseNumberGrid2D<Patch>,
    pub agents_field: SparseGrid2D<Walker>,
    pub dim: (u16, u16),
    pub num_agents: usize,
}

impl Board {
    pub fn new(dim: (u16, u16), num_agents: usize) -> Board {
        Board { step: 0, agents_field: SparseGrid2D::new(dim.0.into(), dim.0.into()), field: DenseNumberGrid2D::new(dim.0.into(), dim.1.into()), dim, num_agents }
    }
}

impl State for Board {
    fn init(&mut self,schedule: &mut krabmaga::engine::schedule::Schedule) {
        self.step = 0;
        let mut rng = rand::thread_rng();

        for _ in 0..self.num_agents {
            let x: u16 = rng.gen_range(0..self.dim.0);
            let y: u16 = rng.gen_range(0..self.dim.1);

            let id: u32 = rng.gen();

            let agent = Walker {
                id,
                pos: Int2D { x: x.into(), y: y.into() }
            };
            // Put the agent in your state
            schedule.schedule_repeating(Box::new(agent), 0., 0);
        }

        let mut id = 0;
        for i in 0..self.dim.0 {
            for j in 0..self.dim.1 {
                let food: u16 = rng.gen_range(0..=1);
                let patch: Patch;
                if food == 1 {
                    patch = Patch::new(id, EnvItem::food);
                } else {
                    patch = Patch::new(id, EnvItem::land);
                }
                id += 1;
                let pos = Int2D { x: i.into(), y: j.into() };
                self.field.set_value_location(patch, &pos);
            }
        } 
    }

    fn after_step(&mut self,schedule: &mut krabmaga::engine::schedule::Schedule) {
        self.step += 1
    }

    fn as_any(&self) ->  &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) ->  &mut dyn std::any::Any {
        self
    }

    fn as_state(&self) ->  &dyn State {
        self
    }

    fn as_state_mut(&mut self) ->  &mut dyn State {
        self
    }

    fn update(&mut self, step:u64) {
        if step == 0 {
            self.field.lazy_update();
        }
        self.agents_field.lazy_update();
    }

    fn reset(&mut self) {
        self.step = 0;
        self.field = DenseNumberGrid2D::new(self.dim.0.into(), self.dim.1.into());
        self.agents_field = SparseGrid2D::new(self.dim.0.into(), self.dim.1.into());
    }
}

