use dfdx::prelude::Module;
use dfdx::prelude::ReLU;
use dfdx::prelude::Linear as LinearT;
use dfdx::nn::modules::Linear;
use dfdx::shapes::{Rank1, Const};
use dfdx::tensor::{Tensor, Cpu, TensorFromVec};
use krabmaga::engine::{agent::Agent,location::Int2D};
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::state::State;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::hash::{Hash, Hasher};
use crate::model::action::Action;

use super::board::Board;
use super::env_item::EnvItem;
use super::observation::Observation;

pub type Policy = (Linear<8,16,f32,Cpu>, ReLU, Linear<16,4,f32,Cpu>);
pub type PolicyTemplate = (LinearT<8,16>, ReLU, LinearT<16,4>);

#[derive(Clone, Copy)]
pub struct Walker {
    pub id: u32,
    pub pos: Int2D
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West
        }
    }
}

impl Agent for Walker {
    fn step(&mut self, state: &mut dyn krabmaga::engine::state::State) {

        let obs = self.make_obs(state);
        let act = self.do_action(state, obs);

        let state = state.as_any().downcast_ref::<Board>().unwrap();
        let item = state.field.get_objects(&self.pos).unwrap()[0].env_item;
        match item {
            EnvItem::Land => {
                match act {
                    Action::North => self.pos.y += 1,
                    Action::East => self.pos.x += 1,
                    Action::South => self.pos.y -= 1,
                    Action::West => self.pos.x -= 1
                }
            },
            EnvItem::Tree => {},
            EnvItem::Sweet => {}
        }

        if self.pos.x > state.dim.0.into() {
            self.pos.x = state.dim.0.into()
        } else if self.pos.x < 1 {
            self.pos.x = 1            
        }
        if self.pos.y > state.dim.1.into() {
            self.pos.y = state.dim.1.into()
        } else if self.pos.y < 1 {
            self.pos.y = 1           
        }

        state
            .agents_field
            .set_object_location(*self, &Int2D { x: self.pos.x, y: self.pos.y });
    }

    // removes agent from simulation
    fn is_stopped(&mut self, state: &mut dyn State) -> bool {
        let state = state.as_any().downcast_ref::<Board>().unwrap();
        if let EnvItem::Sweet = state.field.get_objects(&self.pos).unwrap()[0].env_item {
            true
        } else {
            false
        }
    }
}

impl Location2D<Int2D> for Walker {
    fn get_location(self) -> Int2D {
        self.pos
    }

    fn set_location(&mut self, pos: Int2D) {
        self.pos = pos;
    }
}

// impl fmt::Display for Walker {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.id)
//     }
// }

impl Eq for Walker {}

impl PartialEq for Walker {
    fn eq(&self, other: &Walker) -> bool {
        self.id == other.id
    }
}

impl Hash for Walker {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}


impl Walker {
    pub fn make_obs(&self, state: &mut dyn krabmaga::engine::state::State) -> Observation {
        let state = state.as_any().downcast_ref::<Board>().unwrap();
        let dev: Cpu = Default::default();
        let mut view_build = Vec::new();
        let x = self.pos.x;
        let y = self.pos.y;
        let surround = vec![(x-1,y+1),(x,y+1),(x+1,y+1),(x-1,y),(x+1,y),(x-1,y-1),(x,y-1),(x+1,y-1)];
        for (x,y) in surround {
            if let Some(obj) = state.field.get_objects(&Int2D{x,y}) {
                if let EnvItem::Tree = obj[0].env_item {
                    view_build.push(1.0)
                } else {
                    view_build.push(0.0)
                }
            } else {
                view_build.push(0.0)
            }
        }
        let view: Tensor<Rank1<8>,f32,Cpu> = dev.tensor_from_vec(view_build,(Const,));
        
        Observation { view, }
    }

    pub fn do_action(&self, state: &mut dyn krabmaga::engine::state::State, obs: Observation) -> Action {
        let state = state.as_any().downcast_ref::<Board>().unwrap();
        let policy = state.policies.get(&self.id).expect("policy held fr each agent");
        let y = policy.forward(obs.view);
        let max_idx = y.as_vec()
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap();
        match max_idx {
            0 => Action::North,
            1 => Action::East,
            2 => Action::South,
            _ => Action::West 
        }
    }
}