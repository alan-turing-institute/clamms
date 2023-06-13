use super::action::Action;
use super::agent_state::AgentState;
use super::board::Board;
use super::environment::{EnvItem, Resource};
use super::history::SAR;
use super::inventory::Inventory;
use super::reward::Reward;
use crate::config::{
    FOOD_ACQUIRE_RATE, FOOD_CONSUME_RATE, FOOD_MAX_INVENTORY, WATER_ACQUIRE_RATE,
    WATER_CONSUME_RATE, WATER_MAX_INVENTORY,
};
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::{agent::Agent, location::Int2D};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Forager {
    pub id: u32,
    pub pos: Int2D,
    food: i32,
    water: i32,
}

#[derive(Debug)]
/// Direction of movement.
pub enum Direction {
    North,
    East,
    South,
    West,
    Stationary,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=4) {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => Direction::Stationary,
        }
    }
}

impl Inventory for Forager {
    /// Returns the amount of a given resource in the inventory.
    fn count(&self, resource: &Resource) -> i32 {
        match resource {
            Resource::Food => self.food,
            Resource::Water => self.water,
        }
    }

    fn acquire(&mut self, resource: &Resource, quantity: i32) {
        match resource {
            Resource::Food => self.food += quantity,
            Resource::Water => self.water += quantity,
        }
        self.food = self.food.min(FOOD_MAX_INVENTORY);
        self.water = self.water.min(WATER_MAX_INVENTORY);
    }

    // fn consume(&mut self, resource: &Resource, quantity: i32) {
    //     todo!()
    // }
}

impl Agent for Forager {
    fn step(&mut self, state: &mut dyn krabmaga::engine::state::State) {
        // now downcasting to a mutable reference
        let state = state.as_any_mut().downcast_mut::<Board>().unwrap();
        let item = state.resource_grid.get_objects(&self.pos).unwrap()[0].env_item;

        // record current agent state
        let agent_state = AgentState {
            food: self.food,
            water: self.water,
            // TODO: placeholder waiting for routing work
            food_dist: 0,
            water_dist: 0,
            last_action: state
                .agent_histories
                .get(&self.id)
                .expect("HashMap initialised for all agents")
                .last_action(),
        };

        match item {
            EnvItem::Land => {}
            EnvItem::Resource(Resource::Food) => self.acquire(&Resource::Food, FOOD_ACQUIRE_RATE),
            EnvItem::Resource(Resource::Water) => {
                self.acquire(&Resource::Water, WATER_ACQUIRE_RATE)
            }
        }

        // select action from policy
        let action = Action::ToFood;

        let dir: Direction = rand::random();
        match dir {
            Direction::North => self.pos.y += 1,
            Direction::East => self.pos.x += 1,
            Direction::South => self.pos.y -= 1,
            Direction::West => self.pos.x -= 1,
            Direction::Stationary => (),
        }

        // inventory reduced at each time step
        self.consume(&Resource::Food, FOOD_CONSUME_RATE);
        self.consume(&Resource::Water, WATER_CONSUME_RATE);

        // Clamp positions to be 1 <= pos < dim
        self.pos.x = self.pos.x.clamp(1, (state.dim.0 - 1).into());
        self.pos.y = self.pos.y.clamp(1, (state.dim.1 - 1).into());

        state.agent_grid.set_object_location(
            *self,
            &Int2D {
                x: self.pos.x,
                y: self.pos.y,
            },
        );

        // push (s_n, a_n, r_n+1) to history
        state
            .agent_histories
            .get_mut(&self.id)
            .expect("HashMap initialised for all agents")
            .push(SAR::new(
                agent_state,
                action,
                Reward::from_inv_count_linear(self.food, self.water),
            ))
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
