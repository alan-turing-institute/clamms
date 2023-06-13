use krabmaga::engine::fields::field_2d::Location2D;
use rand::distributions::{Bernoulli, Distribution};
use krabmaga::engine::{agent::Agent, location::Int2D, state::State};
use super::board::{Board, Patch};
use super::environment::Resource;
use crate::model::forager::Direction;

pub trait Router : Position {
    
    /// Gets an appropriate direction of movement towards a specified resource.
    fn try_move_towards(&self, resource: &Resource, state: &dyn State) -> Direction;

    /// Finds the coordinates of the nearest specified resource.
    fn find_nearest(&self, resource: &Resource, state: &dyn State, horizon: Option<f32>) -> Option<Int2D> {
        
        // TODO: use horizon

        let state = state.as_any().downcast_ref::<Board>().unwrap();
        let agent_pos = &self.get_position();
        let mut nearest: Option<Int2D> = None;
        // let mut best_dist: u32 = (state.dim.0 * state.dim.1) as u32;
        let mut id: u32 = 0;
        while let Some(resource_pos) = state.resource_grid.get_location(&resource.to_patch(id) ) {
            if let Some(h) = horizon {
                if sight_distance(agent_pos, &resource_pos) > h {
                    continue;
               }
            }
            if let Some(nearest_pos) = nearest {
                let dist = step_distance(&agent_pos, &resource_pos);
                if dist < step_distance(&agent_pos, &nearest_pos) {
                    nearest = Some(resource_pos);
                }
            } else {
                nearest = Some(resource_pos);
            }
            id += 1;
        }
        nearest
    }
}

pub trait Position {
    fn get_position(&self) -> Int2D;
}

fn coin_flip() -> bool {
    let d = Bernoulli::new(0.5).unwrap();
    d.sample(&mut rand::thread_rng())
}

/// Computes the number of steps to move from a to b.
fn step_distance(a: &Int2D, b: &Int2D) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()).try_into().unwrap()
}

/// Computes the number of steps to move from a to b.
fn sight_distance(a: &Int2D, b: &Int2D) -> f32 {
    f32::sqrt(((a.x - b.x)^2 + (a.y - b.y)^2) as f32)
}

/// Decides an appropriate direction to move towards a target.
pub fn move_towards(pos: &Int2D, target: &Int2D) -> Direction {

    if pos.eq(target) {
        return Direction::Stationary
    }
    if pos.x < target.x {
        if pos.y == target.y {
            return Direction::East
        }
        if pos.y < target.y {
            // flip coin for East or North
            if coin_flip() { return Direction::East } else { return Direction::North }
        } else {
            // flip coin for East or South
            if coin_flip() { return Direction::East } else { return Direction::South }
        }
    }
    if pos.x > target.x {
        if pos.y == target.y {
            return Direction::West
        }
        if pos.y < target.y {
            // flip coin for West or North
            if coin_flip() { return Direction::West } else { return Direction::North }
        } else {
            // flip coin for West or South
            if coin_flip() { return Direction::West } else { return Direction::South }
        }
    }
    if pos.y < target.y {
        return Direction::North
    }
    Direction::South
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_move_towards() {
        let target = Int2D {x: 10, y: 10};

        let pos = Int2D {x: 10, y: 10};
        assert_eq!(move_towards(&pos, &target), Direction::Stationary);

        let pos = Int2D {x: 1, y: 10};
        assert_eq!(move_towards(&pos, &target), Direction::East);

        let pos = Int2D {x: 11, y: 10};
        assert_eq!(move_towards(&pos, &target), Direction::West);

        let pos = Int2D {x: 10, y: 5};
        assert_eq!(move_towards(&pos, &target), Direction::North);

        let pos = Int2D {x: 10, y: 12};
        assert_eq!(move_towards(&pos, &target), Direction::South);
        
        let pos = Int2D {x: 4, y: 8};
        let result = move_towards(&pos, &target);
        assert!(result == Direction::North || result == Direction::East);

        let pos = Int2D {x: 4, y: 20};
        let result = move_towards(&pos, &target);
        assert!(result == Direction::South || result == Direction::East);

        let pos = Int2D {x: 14, y: 8};
        let result = move_towards(&pos, &target);
        assert!(result == Direction::North || result == Direction::West);

        let pos = Int2D {x: 11, y: 18};
        let result = move_towards(&pos, &target);
        assert!(result == Direction::South || result == Direction::West);
    }


}
