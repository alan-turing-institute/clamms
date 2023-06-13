use rand::distributions::{Bernoulli, Distribution};
use crate::model::forager::Direction;
use krabmaga::engine::{agent::Agent, location::Int2D};

pub fn coin_flip() -> bool {
    let d = Bernoulli::new(0.5).unwrap();
    d.sample(&mut rand::thread_rng())
}

pub fn move_towards(pos: &Int2D, target: &Int2D) -> Direction {
    /// Decides an appropriate direction to move towards a target.

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
        let pos = Int2D {x: 10, y: 10};
        let target = Int2D {x: 10, y: 10};
        assert_eq!(move_towards(&pos, &target), Direction::Stationary);
    }


}
