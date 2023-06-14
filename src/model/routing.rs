use super::board::{Board, Patch};
use super::environment::Resource;
use crate::model::forager::Direction;
use krabmaga::cfg_if::cfg_if;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::{location::Int2D, state::State};
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::StdRng;



pub trait Router: Position {
    
    /// Gets an appropriate direction of movement towards a specified resource.
    fn try_move_towards_resource(&self, resource: &Resource, state: &mut dyn State) -> Option<Direction>;

    fn get_resource_locations(&self, resource: &Resource, state: &mut dyn State) -> Vec<Int2D> {
        let state = state.as_any().downcast_ref::<Board>().unwrap();
        state
            .resource_locations
            .get(resource)
            .expect("HashMap initialised for all resource types")
            .to_owned()
    }
    
    // fn get_agent_locations(&self, state: &mut dyn State) -> Vec<Int2D>{
    //     let state = state.as_any().downcast_ref::<Board>().unwrap();
    //     let mut agent_locations: Vec<Int2D>;
    //     for agent in state.forager_grid {
    //             // iter_objects(closure) {
    //         agent_locations.append(state.forager_grid.get_location(agent));
    //     }
    //     agent_locations
    // }

    /// Finds the coordinates of the nearest specified resource.
    fn find_nearest(
        &self,
        resource: &Resource,
        state: &dyn State,
        horizon: Option<f32>,
    ) -> Option<Int2D> {
        let state = state.as_any().downcast_ref::<Board>().unwrap();
        let agent_pos = &self.get_position();
        let mut nearest: Option<Int2D> = None;

        let resource_locations = state
            .resource_locations
            .get(resource)
            .expect("HashMap initialised for all resource types");
        for resource_pos in resource_locations {
            if let Some(h) = horizon {
                if sight_distance(agent_pos, resource_pos) > h {
                    continue;
                }
            }
            if let Some(nearest_pos) = nearest {
                let dist = step_distance(agent_pos, resource_pos);
                if dist < step_distance(agent_pos, &nearest_pos) {
                    nearest = Some(resource_pos.to_owned());
                }
            } else {
                nearest = Some(resource_pos.to_owned());
            }
        }
        nearest
    }
}

pub trait Position {
    fn get_position(&self) -> Int2D;
}

fn coin_flip(rng: &mut StdRng) -> bool {
    let d = Bernoulli::new(0.5).unwrap();
    d.sample(rng)
}

/// Computes the number of steps to move from a to b.
fn step_distance(a: &Int2D, b: &Int2D) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()).try_into().unwrap()
}

/// Computes the straight line distance from a to b.
fn sight_distance(a: &Int2D, b: &Int2D) -> f32 {
    f32::sqrt(((a.x - b.x).pow(2) + (a.y - b.y).pow(2)) as f32)
}

/// Decides an appropriate direction to move towards a target.
pub fn move_towards(pos: &Int2D, target: &Int2D, rng: &mut StdRng) -> Option<Direction> {
    if pos.eq(target) {
        return None;
    }
    if pos.x < target.x {
        if pos.y == target.y {
            return Some(Direction::East);
        }
        if pos.y < target.y {
            // flip coin for East or North
            if coin_flip(rng) {
                return Some(Direction::East);
            } else {
                return Some(Direction::North);
            }
        } else {
            // flip coin for East or South
            if coin_flip(rng) {
                return Some(Direction::East);
            } else {
                return Some(Direction::South);
            }
        }
    }
    if pos.x > target.x {
        if pos.y == target.y {
            return Some(Direction::West);
        }
        if pos.y < target.y {
            // flip coin for West or North
            if coin_flip(rng) {
                return Some(Direction::West);
            } else {
                return Some(Direction::North);
            }
        } else {
            // flip coin for West or South
            if coin_flip(rng) {
                return Some(Direction::West);
            } else {
                return Some(Direction::South);
            }
        }
    }
    if pos.y < target.y {
        return Some(Direction::North);
    }
    Some(Direction::South)
}

#[cfg(test)]
mod tests {
    use crate::{model::forager::Forager, config::core_config};

    use super::*;
    use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
    use rand::SeedableRng;

    #[test]
    fn test_sight_dist() {
        assert_eq!(
            sight_distance(&Int2D { x: 0, y: 0 }, &Int2D { x: 4, y: 3 }),
            5.
        )
    }

    #[test]
    fn test_move_towards() {
        let mut rng = StdRng::from_entropy();
        let target = Int2D { x: 10, y: 10 };

        let pos = Int2D { x: 10, y: 10 };
        assert_eq!(move_towards(&pos, &target, &mut rng), None);

        let pos = Int2D { x: 1, y: 10 };
        assert_eq!(move_towards(&pos, &target, &mut rng), Some(Direction::East));

        let pos = Int2D { x: 11, y: 10 };
        assert_eq!(move_towards(&pos, &target, &mut rng), Some(Direction::West));

        let pos = Int2D { x: 10, y: 5 };
        assert_eq!(
            move_towards(&pos, &target, &mut rng),
            Some(Direction::North)
        );

        let pos = Int2D { x: 10, y: 12 };
        assert_eq!(
            move_towards(&pos, &target, &mut rng),
            Some(Direction::South)
        );

        let pos = Int2D { x: 4, y: 8 };
        let result = move_towards(&pos, &target, &mut rng);
        assert!(result == Some(Direction::North) || result == Some(Direction::East));

        let pos = Int2D { x: 4, y: 20 };
        let result = move_towards(&pos, &target, &mut rng);
        assert!(result == Some(Direction::South) || result == Some(Direction::East));

        let pos = Int2D { x: 14, y: 8 };
        let result = move_towards(&pos, &target, &mut rng);
        assert!(result == Some(Direction::North) || result == Some(Direction::West));

        let pos = Int2D { x: 11, y: 18 };
        let result = move_towards(&pos, &target, &mut rng);
        assert!(result == Some(Direction::South) || result == Some(Direction::West));
    }

    // #[test]
    // fn test_get_agent_locations() {
        
    //     let dim: (u16, u16) = (10, 10);
    //     let mut board = Board::new(dim, 0);
    //     let agent_grid: DenseGrid2D<Forager> = DenseGrid2D::new(dim.0.into(), dim.0.into());
    //     let mut positions: Vec<Int2D> = Vec::new();
    //     positions.push(Int2D { x: 4, y: 8 });
    //     positions.push(Int2D { x: 1, y: 2 });
    //     for p in positions {
    //         let agent = Forager::new(
    //             0,
    //             p,
    //             core_config().agent.INIT_FOOD,
    //             core_config().agent.INIT_WATER,
    //         );
    //         agent_grid.set_object_location(agent, &agent.pos)
    //     }

    //     let result = get_agent_locations();

    // }
}
