use super::action::Action;
use super::agent_state::{AgentState, DiscrRep};
use super::board::Board;
use super::environment::{EnvItem, Resource};
use super::history::SAR;
use super::inventory::Inventory;
use super::policy::Policy;
use super::reward::Reward;
use super::routing::{
    get_resource_locations, get_trader_locations, move_towards, Position, Router,
};
use super::trader::Trader;
use crate::config::core_config;
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::state::State;
use krabmaga::engine::{agent::Agent, location::Int2D};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Forager {
    id: u32,
    pub pos: Int2D,
    food: i32,
    water: i32,
}

#[derive(Debug, PartialEq)]
/// Direction of movement.
pub enum Direction {
    North,
    East,
    South,
    West,
    // Stationary,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
            // _ => Direction::Stationary,
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
        self.food = self.food.min(core_config().agent.FOOD_MAX_INVENTORY);
        self.water = self.water.min(core_config().agent.WATER_MAX_INVENTORY);
    }

    // fn consume(&mut self, resource: &Resource, quantity: i32) {
    //     todo!()
    // }
}

impl Policy for Forager {
    fn chose_action(&self, state: &mut dyn State, agent_state: &AgentState) -> Action {
        let state = state.as_any_mut().downcast_mut::<Board>().unwrap();
        state
            .model
            .sample_action_by_id(self.id, &agent_state.representation(), &mut state.rng)
        // if agent_state.food < agent_state.water {
        //     Action::ToFood
        // } else {
        //     Action::ToWater
        // }
    }
}

impl Agent for Forager {
    fn step(&mut self, state: &mut dyn State) {
        // now downcasting to a mutable reference
        let state = state.as_any_mut().downcast_mut::<Board>().unwrap();

        // observe current agent state
        let agent_state = self.agent_state(state);

        // select action from policy
        let action = self.chose_action(state, &agent_state);

        // route agent based on action
        let route = match action {
            Action::ToFood => self.try_move_towards_resource(&Resource::Food, state, None),
            Action::ToWater => self.try_move_towards_resource(&Resource::Water, state, None),
            Action::ToAgent => self.try_move_towards_agent(state, None),
            _ => None,
        };

        // TODO: consider moving to a new update_position method:
        if let Some(dir) = route {
            match dir {
                Direction::North => self.pos.y += 1,
                Direction::East => self.pos.x += 1,
                Direction::South => self.pos.y -= 1,
                Direction::West => self.pos.x -= 1,
            }
            // Clamp positions to be 1 <= pos < dim
            self.pos.x = self.pos.x.clamp(1, (state.dim.0 - 1).into());
            self.pos.y = self.pos.y.clamp(1, (state.dim.1 - 1).into());
        }

        // update agent position (executing action)
        state.agent_grid.set_object_location(
            // TODO: fix to not use a trader inside forager
            Trader::new(*self),
            &Int2D {
                x: self.pos.x,
                y: self.pos.y,
            },
        );
        // END OF update_position.

        // resources depleted automatically after taking an action (even if Action::Stationary)
        self.consume(&Resource::Food, core_config().agent.FOOD_CONSUME_RATE);
        self.consume(&Resource::Water, core_config().agent.WATER_CONSUME_RATE);

        // if now on a resource, gather the resource
        let item = state.resource_grid.get_objects(&self.pos).unwrap()[0].env_item;
        match item {
            EnvItem::Land => {}
            EnvItem::Resource(Resource::Food) => {
                self.acquire(&Resource::Food, core_config().agent.FOOD_ACQUIRE_RATE)
            }
            EnvItem::Resource(Resource::Water) => {
                self.acquire(&Resource::Water, core_config().agent.WATER_ACQUIRE_RATE)
            }
        }

        // push (s_n, a_n, r_n+1) to history
        state
            .agent_histories
            .get_mut(&self.id())
            .expect("HashMap initialised for all agents")
            .push(SAR::new(
                agent_state,
                action.clone(),
                Reward::from_inv_count_linear(self.food, self.water),
            ));

        // if self.id == 0 {
        //     println!(
        //         "agent: {:?}, food: {:?}, water: {:?}, act: {:?}, pos: {}",
        //         self.id,
        //         self.food,
        //         self.water,
        //         action,
        //         self.get_position()
        //     );
        // }
    }
}

impl Position for Forager {
    fn get_position(&self) -> Int2D {
        self.pos.to_owned()
    }
}

impl Router for Forager {}

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

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn agent_state(&self, state: &dyn krabmaga::engine::state::State) -> AgentState {
        let min_steps_to_food = self.min_steps_to(get_resource_locations(&Resource::Food, state));
        let min_steps_to_water = self.min_steps_to(get_resource_locations(&Resource::Water, state));

        let min_steps_to_trader = self.min_steps_to(get_trader_locations(state));

        AgentState {
            food: self.food,
            water: self.water,
            min_steps_to_food,
            min_steps_to_water,
            min_steps_to_trader,
            // TODO: placeholder waiting for routing work
            // last_action: state
            //     .agent_histories
            //     .get(&self.id)
            //     .expect("HashMap initialised for all agents")
            //     .last_action(),
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
