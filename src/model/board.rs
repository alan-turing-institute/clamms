use super::agent_api::AgentAPI;
use super::environment::Resource;
use super::history::History;
use super::trader::Trader;
use crate::config::core_config;

use super::action::Action;
use super::agent_state::{AgentState, AgentStateItems, InvLevel};
use super::tabular_rl::SARSAModel;
use super::{environment::EnvItem, forager::Forager};
use itertools::Itertools;
use krabmaga::cfg_if::cfg_if;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::{location::Int2D, state::State};
use krabmaga::HashMap;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug)]
pub struct Patch {
    pub id: u32,
    pub env_item: EnvItem,
}

impl Patch {
    pub fn new(id: u32, env_item: EnvItem) -> Self {
        Patch { id, env_item }
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

/// Clamms version of Int2D with trait implementations and conversion to Int2D
#[derive(Clone, Copy, Serialize, Deserialize, Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct ClammsInt2D {
    pub x: i32,
    pub y: i32,
}

impl From<ClammsInt2D> for Int2D {
    fn from(value: ClammsInt2D) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
///
pub fn read_resource_locations(input: &str) -> BTreeMap<Resource, Vec<Int2D>> {
    serde_json::from_str::<BTreeMap<Resource, Vec<ClammsInt2D>>>(input)
        .unwrap()
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().map(Int2D::from).collect()))
        .collect()
}

pub fn example_board(dim: (u16, u16)) -> BTreeMap<Resource, Vec<ClammsInt2D>> {
    let mut map = BTreeMap::new();
    map.insert(Resource::Food, vec![]);
    map.insert(Resource::Water, vec![]);
    for i in 1..7 {
        for j in 1..7 {
            let v = map.get_mut(&Resource::Food).unwrap();
            v.push(ClammsInt2D { x: i, y: j });
        }
    }
    let mut rng = StdRng::seed_from_u64(1);
    let mut river_width = 0i32;
    for j in 1..dim.1 {
        river_width += rng.gen_range(-2..=2);
        for i in (dim.0 / 2 - 2)..=(dim.0 / 2 + 2) {
            let v = map.get_mut(&Resource::Water).unwrap();
            v.push(ClammsInt2D {
                x: (i as i32 - 2 + river_width).into(),
                y: j.into(),
            });
        }
    }
    map
}

// TODO: add a fast lookup by location for resources
pub struct Board {
    pub step: u64,
    pub resource_grid: DenseGrid2D<Patch>,
    pub agent_grid: DenseGrid2D<Trader>,
    pub dim: (u16, u16),
    pub num_agents: u8,
    pub agent_histories: BTreeMap<u32, History<AgentState, AgentStateItems, InvLevel, Action>>,
    pub resource_locations: BTreeMap<Resource, Vec<Int2D>>,
    pub rng: StdRng,
    pub model: SARSAModel<AgentState, AgentStateItems, InvLevel, Action>,
    pub loaded_map: bool,
    pub has_trading: bool,
    pub traded: HashMap<u32, Option<u32>>,
}

impl Board {
    pub fn new(
        dim: (u16, u16),
        num_agents: u8,
        model: SARSAModel<AgentState, AgentStateItems, InvLevel, Action>,
        has_trading: bool,
    ) -> Board {
        Board {
            step: 0,
            agent_grid: DenseGrid2D::new(dim.0.into(), dim.1.into()),
            resource_grid: DenseGrid2D::new(dim.0.into(), dim.1.into()),
            dim,
            num_agents,
            agent_histories: BTreeMap::new(),
            resource_locations: BTreeMap::new(),
            rng: StdRng::from_entropy(),
            model,
            loaded_map: false,
            has_trading,
            traded: HashMap::new(),
        }
    }
    pub fn new_with_seed(
        dim: (u16, u16),
        num_agents: u8,
        seed: u64,
        model: SARSAModel<AgentState, AgentStateItems, InvLevel, Action>,
        has_trading: bool,
    ) -> Board {
        Board {
            step: 0,
            agent_grid: DenseGrid2D::new(dim.0.into(), dim.1.into()),
            resource_grid: DenseGrid2D::new(dim.0.into(), dim.1.into()),
            dim,
            num_agents,
            agent_histories: BTreeMap::new(),
            resource_locations: BTreeMap::new(),
            rng: StdRng::seed_from_u64(seed),
            model,
            loaded_map: false,
            has_trading,
            traded: HashMap::new(),
        }
    }
    pub fn new_with_seed_resources(
        dim: (u16, u16),
        num_agents: u8,
        seed: u64,
        map_locations: &str,
        model: SARSAModel<AgentState, AgentStateItems, InvLevel, Action>,
        has_trading: bool,
    ) -> Board {
        let path =
            std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(map_locations);
        let resource_locations = read_resource_locations(&std::fs::read_to_string(path).unwrap());

        Board {
            step: 0,
            agent_grid: DenseGrid2D::new(dim.0.into(), dim.0.into()),
            resource_grid: DenseGrid2D::new(dim.0.into(), dim.1.into()),
            dim,
            num_agents,
            agent_histories: BTreeMap::new(),
            resource_locations,
            rng: StdRng::seed_from_u64(seed),
            loaded_map: true,
            model,
            has_trading,
            traded: HashMap::new(),
        }
    }

    /// Randomly inits agents.
    fn generate_agents_random(&mut self, schedule: &mut Schedule) {
        for n in 0..self.num_agents {
            let x: u16 = self.rng.gen_range(1..self.dim.0);
            let y: u16 = self.rng.gen_range(1..self.dim.1);

            let id: u32 = n.into();

            let agent = Trader::new(Forager::new(
                id,
                Int2D {
                    x: x.into(),
                    y: y.into(),
                },
                core_config().agent.INIT_FOOD,
                core_config().agent.INIT_WATER,
            ));

            // Init empty history
            self.agent_histories.insert(id, History::new());

            // Put the agent in your state
            schedule.schedule_repeating(Box::new(agent), 0., 0);

            // Set agent location
            self.agent_grid
                .set_object_location(agent, &agent.forager.pos)
        }
    }

    /// Randomly sets resource locations from config.
    fn set_resources_random(&mut self) {
        Resource::iter().for_each(|resource| {
            self.resource_locations.insert(resource, Vec::new());
        });
        let mut id = 0;
        for i in 0..self.dim.0 {
            for j in 0..self.dim.1 {
                let pos = Int2D {
                    x: i.into(),
                    y: j.into(),
                };
                let item = self.rng.gen();
                let patch = Patch::new(id, item);
                self.resource_grid.set_object_location(patch, &pos);
                if let EnvItem::Resource(resource) = patch.env_item {
                    self.resource_locations
                        .get_mut(&resource)
                        .expect("HashMap initialised for all resource types")
                        .push(pos.to_owned());
                }
                id += 1;
            }
        }
    }

    /// Sets resource locations based on loaded map.
    fn set_resources_from_map(&mut self) {
        let mut resource_lookup: HashMap<Int2D, Resource> = HashMap::new();
        self.resource_locations.iter().for_each(|(&res, v)| {
            for loc in v.iter() {
                resource_lookup.insert(*loc, res);
            }
        });
        let mut id = 0;
        for i in 0..self.dim.0 {
            for j in 0..self.dim.1 {
                let pos = Int2D {
                    x: i.into(),
                    y: j.into(),
                };

                let item = if let Some(resource) = resource_lookup.get(&pos) {
                    EnvItem::Resource(*resource)
                } else if self.rng.gen::<f32>() < core_config().world.LAND_PROP {
                    EnvItem::Land
                } else {
                    EnvItem::Bush
                };

                let patch = Patch::new(id, item);
                self.resource_grid.set_object_location(patch, &pos);
                id += 1;
            }
        }
    }
}

impl State for Board {
    fn init(&mut self, schedule: &mut Schedule) {
        // Init step
        self.step = 0;
        // Generate agents
        self.generate_agents_random(schedule);
        // Generate and set resource grid
        if self.loaded_map {
            self.set_resources_from_map();
        } else {
            self.set_resources_random();
        }
        // Call lazy_update on the resource grid
        self.resource_grid.lazy_update();
    }

    fn before_step(&mut self, _: &mut krabmaga::engine::schedule::Schedule) {}

    fn after_step(&mut self, schedule: &mut krabmaga::engine::schedule::Schedule) {
        // TODO: add random ordering using board.rng to events in scheduler so that agents are picked
        // in a different random order each time during step.

        // Updates as state
        let step: i32 = i32::try_from(self.step).unwrap();

        // Update board model
        let board = self.as_any_mut().downcast_mut::<Board>().unwrap();
        board.model.step(step, &board.agent_histories);

        // TODO: add better dashboard statistics for agents/optimization
        // Simple report of mean reward over last 100
        let traj = &board.agent_histories.get(&0).unwrap().trajectory;
        let recent_len = 100;
        let recent_traj = &traj[(traj.len().max(recent_len) - recent_len)..traj.len()];
        if core_config().simulation.VERBOSITY > 0 {
            println!(
                "Mean reward (over last 100 steps) for agent 0: {} at step: {step}",
                recent_traj.iter().map(|sar| sar.reward.val).sum::<i32>()
                    / i32::try_from(recent_traj.len()).unwrap()
            );
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn update(&mut self, step: u64) {
        // The agent_grid updated at end of timestep so set_object_location() is switched to "read" from "write"
        self.agent_grid.lazy_update();
        // Clear traded lookup
        self.traded.clear();
        self.step = step;
    }

    fn reset(&mut self) {
        self.step = 0;
        self.resource_grid = DenseGrid2D::new(self.dim.0.into(), self.dim.1.into());
        self.agent_grid = DenseGrid2D::new(self.dim.0.into(), self.dim.1.into());
    }
}

// Additional API for accessing board.
cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        impl AgentAPI<Trader> for Board {
            fn get_agent_by_id(&self, id: &u32) -> Trader {
                self.agent_grid
                    .get(&Trader::dummy(*id))
                    .expect("get agent by id")
            }
            fn get_agents(&self) -> Vec<Trader> {
                self.agent_grid.obj2loc.keys().iter().map(|&k|k.to_owned()).collect()
            }
        }
    } else {
        impl AgentAPI<Trader> for Board {
            fn get_agent_by_id(&self, id: &u32) -> Trader {
                let loc = &self.agent_grid.get_location(&Trader::dummy(*id)).unwrap();
                let traders: Vec<Trader> = self.agent_grid.get_objects(loc).unwrap();
                *traders.into_iter().filter(|trader| trader.id() == *id).collect_vec().first().unwrap()
            }
            fn get_agents(&self) -> Vec<Trader> {
                let mut traders: Vec<Trader> = Vec::new();
                for i in  0..self.dim.0 {
                    for j in 0..self.dim.1 {
                        // Gets objects from "read" state (start of time step)
                        if let Some(mut traders_at_loc) = self.agent_grid.get_objects(&Int2D {x: i.into(), y: j.into() }) {
                                traders.append(&mut traders_at_loc);
                            }
                        }
                    }

                traders
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use krabmaga::engine::schedule::Schedule;

    use crate::model::{init, inventory::Inventory};

    use super::*;

    trait TestInit {
        fn init_with_test_agents(&mut self, schedule: &mut krabmaga::engine::schedule::Schedule);
    }

    impl TestInit for Board {
        fn init_with_test_agents(&mut self, schedule: &mut krabmaga::engine::schedule::Schedule) {
            self.step = 0;
            let agent1 = Trader::new(Forager::new(0, Int2D { x: 2, y: 2 }, 0, 100));
            let agent2 = Trader::new(Forager::new(1, Int2D { x: 2, y: 1 }, 100, 0));
            let agent3 = Trader::new(Forager::new(2, Int2D { x: 4, y: 5 }, 0, 0));
            self.agent_grid
                .set_object_location(agent1, &agent1.forager.pos);
            self.agent_grid
                .set_object_location(agent2, &agent2.forager.pos);
            self.agent_grid
                .set_object_location(agent3, &agent3.forager.pos);
            self.agent_histories.insert(0, History::new());
            self.agent_histories.insert(1, History::new());
            self.agent_histories.insert(2, History::new());
            schedule.schedule_repeating(Box::new(agent1), 0., 0);
            schedule.schedule_repeating(Box::new(agent2), 0., 0);
            schedule.schedule_repeating(Box::new(agent3), 0., 0);
            let resource_lookup = if !self.loaded_map {
                // Init empty resource locations
                for resource in Resource::iter() {
                    self.resource_locations.insert(resource, Vec::new());
                }
                None
            } else {
                let mut resource_lookup: HashMap<Int2D, Resource> = HashMap::new();
                self.resource_locations.iter().for_each(|(&res, v)| {
                    for loc in v.iter() {
                        resource_lookup.insert(*loc, res);
                    }
                });
                Some(resource_lookup)
            };

            let mut id = 0;
            for i in 0..self.dim.0 {
                for j in 0..self.dim.1 {
                    let pos = Int2D {
                        x: i.into(),
                        y: j.into(),
                    };
                    let item = if let Some(resource_lookup) = resource_lookup.as_ref() {
                        if let Some(resource) = resource_lookup.get(&pos) {
                            EnvItem::Resource(*resource)
                        } else if self.rng.gen::<f32>() < core_config().world.LAND_PROP {
                            EnvItem::Land
                        } else {
                            EnvItem::Bush
                        }
                    } else {
                        self.rng.gen()
                    };

                    let patch = Patch::new(id, item);
                    self.resource_grid.set_object_location(patch, &pos);
                    if !self.loaded_map {
                        if let EnvItem::Resource(resource) = patch.env_item {
                            let v = self
                                .resource_locations
                                .get_mut(&resource)
                                .expect("HashMap initialised for all resource types");
                            v.push(pos.to_owned());
                        }
                    }
                    id += 1;
                }
            }
            // Call lazy_update on the resource grid
            self.resource_grid.lazy_update();
        }
    }

    const TEST_LOCATIONS: &str = r#"{
        "Food": [
          {"x": 19,"y": 19},
          {"x": 19,"y": 18},
          {"x": 18, "y": 19},
          {"x": 18,"y": 18}
        ],
        "Water": [
          {"x": 3,"y": 3},
          {"x": 3,"y": 2},
          {"x": 2,"y": 3},
          {"x": 2,"y": 2}
        ]
      }"#;
    #[test]
    fn test_read_resources() {
        let _ = read_resource_locations(TEST_LOCATIONS);
    }
    #[test]
    fn test_example_board() {
        println!(
            "{}",
            serde_json::to_string(&example_board((42, 42))).unwrap()
        );
    }

    #[test]
    fn test_scheduler_event_ordering() {
        // Add test to confirm/randomize order of events in PriorityQueue
        todo!()
    }

    /// Get inventories of agents on a board.
    fn get_inventories(board: &Board) -> HashMap<u32, (i32, i32)> {
        board
            .get_agents()
            .iter()
            .fold(HashMap::new(), |mut acc, trader| {
                acc.insert(
                    trader.id(),
                    (
                        trader.forager().count(&Resource::Food),
                        trader.forager().count(&Resource::Water),
                    ),
                );
                acc
            })
    }
    /// Get inventories of agents on a board.
    fn get_traders_display(board: &Board) -> HashMap<u32, String> {
        board
            .get_agents()
            .iter()
            .fold(HashMap::new(), |mut acc, trader| {
                acc.insert(trader.id(), format!("{}", trader));
                acc
            })
    }

    #[test]
    fn test_board_update() {
        init();
        // Set-up small board with three agents and no resources within trading radius that will make inverse offers
        let seed = core_config().world.RANDOM_SEED;
        let num_agents = core_config().world.N_AGENTS;
        let dim: (u16, u16) = (core_config().world.WIDTH, core_config().world.HEIGHT);
        let has_trading = core_config().world.HAS_TRADING;
        let model = SARSAModel::new(
            (0..num_agents).map(|n| n.into()).collect(),
            AgentStateItems::iter().collect::<Vec<AgentStateItems>>(),
            InvLevel::iter().collect::<Vec<InvLevel>>(),
            Action::iter().collect::<Vec<Action>>(),
            false,
        );

        let mut board = if let Some(file_name) = &core_config().world.RESOURCE_LOCATIONS_FILE {
            Board::new_with_seed_resources(dim, num_agents, seed, file_name, model, has_trading)
        } else {
            Board::new_with_seed(dim, num_agents, seed, model, has_trading)
        };

        // Use scheduler and run directly once
        let mut schedule: Schedule = Schedule::new();
        board.init_with_test_agents(&mut schedule);

        // Get traders and check resource levels are as expected
        let traders0 = get_traders_display(&board);
        let inv0 = get_inventories(&board);
        println!("t=0 (before update): {:?}", traders0);
        // No traders present before any scheduler step
        assert!(inv0.is_empty());
        // First step
        schedule.step(&mut board);
        let traders1 = get_traders_display(&board);
        let inv1 = get_inventories(&board);
        println!("t=1 (before update): {:?}", traders1);
        assert_eq!(*inv1.get(&0).unwrap(), (-5, 95));
        assert_eq!(*inv1.get(&1).unwrap(), (95, -5));
        assert_eq!(*inv1.get(&2).unwrap(), (-5, -5));
        // Second step
        schedule.step(&mut board);
        let traders2 = get_traders_display(&board);
        let inv2 = get_inventories(&board);
        println!("t=2 (before update): {:?}", traders2);
        assert_eq!(*inv2.get(&0).unwrap(), (-9, 89));
        assert_eq!(*inv2.get(&1).unwrap(), (89, -9));
        assert_eq!(*inv2.get(&2).unwrap(), (-10, -10));
    }
}
