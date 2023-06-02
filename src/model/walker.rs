use krabmaga::engine::{agent::Agent,location::Int2D};
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::state::State;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::hash::{Hash, Hasher};
use super::board::Board;
use core::fmt;


#[derive(Clone,Copy)]
pub struct Walker {
    pub id: u32,
    pub pos: Int2D,
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
        let state = state.as_any().downcast_ref::<Board>().unwrap();
        let dir: Direction = rand::random();
        match dir {
            Direction::North => self.pos.y += 1,
            Direction::East => self.pos.x += 1,
            Direction::South => self.pos.y -= 1,
            Direction::West => self.pos.x -= 1
        }

        if self.pos.x > state.dim.0.into() {
            self.pos.x = state.dim.0.into()
        } else if self.pos.x < 0 {
            self.pos.x = 0            
        }

        if self.pos.y > state.dim.1.into() {
            self.pos.y = state.dim.1.into()
        } else if self.pos.y < 0 {
            self.pos.y = 0            
        }

        state
            .agents_field
            .set_object_location(*self, &Int2D { x: self.pos.x, y: self.pos.y });
    }

    // fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
    //     false
    // }
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