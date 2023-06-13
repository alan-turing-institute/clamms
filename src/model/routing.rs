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
