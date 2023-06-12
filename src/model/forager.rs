use super::board::Board;
use super::environment::{EnvItem, Resource};
use super::inventory::Inventory;
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::state::State;
use krabmaga::engine::{agent::Agent, location::Int2D};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Forager {
    pub id: u32,
    pub pos: Int2D,
    food: i32,
    water: i32,
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
        }
    }
}

impl Inventory for Forager {
    fn count(&self, resource: Resource) -> i32 {
        match resource {
            Resource::Food => self.food,
            Resource::Water => self.water,
        }
    }
}

impl Agent for Forager {
    fn step(&mut self, state: &mut dyn krabmaga::engine::state::State) {
        let state = state.as_any().downcast_ref::<Board>().unwrap();
        let item = state.resource_grid.get_objects(&self.pos).unwrap()[0].env_item;
        match item {
            EnvItem::Land => {
                let dir: Direction = rand::random();
                match dir {
                    Direction::North => self.pos.y += 1,
                    Direction::East => self.pos.x += 1,
                    Direction::South => self.pos.y -= 1,
                    Direction::West => self.pos.x -= 1,
                }
            }
            EnvItem::Resource(_) => {}
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

        state.agent_grid.set_object_location(
            *self,
            &Int2D {
                x: self.pos.x,
                y: self.pos.y,
            },
        );
    }
}

impl Location2D<Int2D> for Forager {
    fn get_location(self) -> Int2D {
        self.pos
    }

    fn set_location(&mut self, pos: Int2D) {
        self.pos = pos;
    }
}

// impl fmt::Display for Forager {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.id)
//     }
// }

impl Eq for Forager {}

impl PartialEq for Forager {
    fn eq(&self, other: &Forager) -> bool {
        self.id == other.id
    }
}

impl Hash for Forager {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Forager {
    pub fn new(id: u32, pos: Int2D, food: i32, water: i32) -> Self {
        Self {
            id,
            pos,
            food,
            water,
        }
    }

    /// Dummy forager for matching just on ID.
    pub fn dummy(id: u32) -> Self {
        Forager {
            id,
            pos: Int2D {
                x: Default::default(),
                y: Default::default(),
            },
            food: 0,
            water: 0,
        }
    }
}
