use super::environment::Resource;
use super::history::History;
use super::trader::{settle_trade_on_counterparty, Trade, Trader};
use crate::config::core_config;

use super::action::Action;
use super::agent_state::{AgentState, AgentStateItems, InvLevel};
use super::tabular_rl::SARSAModel;
use super::{environment::EnvItem, forager::Forager};
use crate::engine::fields::grid_option::GridOption;
use crate::model::inventory::Inventory;
use crate::model::routing::step_distance;
use crate::model::trader;
use itertools::Itertools;
use krabmaga::cfg_if::cfg_if;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::{
    fields::sparse_object_grid_2d::SparseGrid2D, location::Int2D, state::State,
};
use krabmaga::hashbrown::HashSet;
use krabmaga::HashMap;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use strum::IntoEnumIterator;
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
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
    pub agent_histories: HashMap<u32, History<AgentState, AgentStateItems, InvLevel, Action>>,
    pub resource_locations: BTreeMap<Resource, Vec<Int2D>>,
    pub loc2resources: HashMap<Int2D, Resource>,
    pub rng: StdRng,
    pub model: SARSAModel<AgentState, AgentStateItems, InvLevel, Action>,
    pub loaded_map: bool,
    pub has_trading: bool,
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
            agent_histories: HashMap::new(),
            resource_locations: BTreeMap::new(),
            loc2resources: HashMap::new(),
            rng: StdRng::from_entropy(),
            model,
            loaded_map: false,
            has_trading,
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
            agent_histories: HashMap::new(),
            resource_locations: BTreeMap::new(),
            loc2resources: HashMap::new(),
            rng: StdRng::seed_from_u64(seed),
            model,
            loaded_map: false,
            has_trading,
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
        let loc2resources = resource_locations.iter().fold(
            HashMap::<Int2D, Resource>::new(),
            |mut acc, (&resource, locs)| {
                locs.iter().for_each(|loc| {
                    acc.insert(*loc, resource);
                });
                acc
            },
        );

        Board {
            step: 0,
            agent_grid: DenseGrid2D::new(dim.0.into(), dim.0.into()),
            resource_grid: DenseGrid2D::new(dim.0.into(), dim.1.into()),
            dim,
            num_agents,
            agent_histories: HashMap::new(),
            resource_locations,
            loc2resources,
            rng: StdRng::seed_from_u64(seed),
            loaded_map: true,
            model,
            has_trading,
        }
    }
}

impl State for Board {
    fn init(&mut self, schedule: &mut krabmaga::engine::schedule::Schedule) {
        self.step = 0;
        for n in 0..self.num_agents {
            let x: u16 = self.rng.gen_range(1..self.dim.0);
            let y: u16 = self.rng.gen_range(1..self.dim.1);

            let id: u32 = n.into();

            // let agent = Forager::new(
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
        }

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
    }

    fn before_step(&mut self, _: &mut krabmaga::engine::schedule::Schedule) {
        if (self.step > 0) & self.has_trading {
            use super::routing::get_traders;
            // get snapshot of agents inventories
            let traders_pre = get_traders(self);
            let inventories_pre: Vec<(i32, i32)> = traders_pre
                .iter()
                .map(|trader| {
                    (
                        trader.forager().count(&Resource::Food),
                        trader.forager().count(&Resource::Water),
                    )
                })
                .collect();

            // randomly generate an agent trade resolution sequence
            let mut ids: Vec<u32> = (0..self.num_agents.into()).collect();
            ids.shuffle(&mut self.rng);

            // loop through agents resolving trades
            for id in ids {
                let cur = get_agent_by_id(self, &id);

                // Execute trade if available.
                if !cur.offer().is_trivial() {
                    let offer = cur.offer();
                    for trader in get_traders(self) {
                        // only use the id's to keep track of which traders have been
                        // considered rather than using reference to traders that
                        // might change within the loop
                        let trader_id = trader.id();
                        if trader_id != cur.id() {
                            let trader = get_agent_by_id(self, &trader_id);
                            if trader.offer().matched(&offer)
                                && (step_distance(&cur.forager.pos, &trader.forager.pos)
                                    < core_config().trade.MAX_TRADE_DISTANCE)
                            {
                                if core_config().simulation.VERBOSITY > 1 {
                                    println!("Trade between: {} and {}", cur, trader);
                                }
                                let settled_trader = settle_trade_on_counterparty(trader, &offer);
                                // Set object only retains objects with different ID to that being set
                                // as PartialEq is based on ID
                                self.agent_grid.set_object_location(
                                    settled_trader,
                                    &settled_trader.forager.pos,
                                );
                                let settled_cur =
                                    settle_trade_on_counterparty(cur, &offer.invert());
                                self.agent_grid
                                    .set_object_location(settled_cur, &settled_cur.forager.pos);
                            }
                        }
                        // TODO: Should break here if a trade occurs?
                    }
                }
            }

            // re-read from agent grid and compare inventories to pre-trade snapshot
            // get snapshot of agents inventories post trading
            let traders_post = get_traders(self);
            let inventories_post: Vec<(i32, i32)> = traders_post
                .iter()
                .map(|trader| {
                    (
                        trader.forager().count(&Resource::Food),
                        trader.forager().count(&Resource::Water),
                    )
                })
                .collect();
            // println!("pre: {:?}", inventories_pre);
            // println!("post: {:?}", inventories_post);
        }
    }

    fn after_step(&mut self, schedule: &mut krabmaga::engine::schedule::Schedule) {
        self.step += 1;

        // Updates as state
        let mut step: i32 = self.step.try_into().unwrap();
        step -= 1;

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
        // lazy_update stops the field being searchable!
        // TODO: in non-visualization feature, calls to update appear to be a bottleneck (flamegraph).
        // Looks like underlying data structure repeatedly clones the vec of vec grid.
        // Resources now have their own lookup by position. Agents are more complex as they are not
        // fixed.
        // Investigate a fix for updates when running without visualization with trading. A place to
        // start might be to check if the data structures with the visualization feature's grid can
        // also be used in the non-visualization feature.
        //
        // Update: lazy_update() now should be ok as resources are fixed and agent updates remove and
        // set object where the mutation occurs in before_step().
        self.resource_grid.lazy_update();
        self.agent_grid.lazy_update();
    }

    fn reset(&mut self) {
        self.step = 0;
        self.resource_grid = DenseGrid2D::new(self.dim.0.into(), self.dim.1.into());
        self.agent_grid = DenseGrid2D::new(self.dim.0.into(), self.dim.1.into());
    }
}

// TODO: refactor into a trait to provide additional API on DenseGrid2D that is needed
cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        pub fn get_agent_by_id(board: &mut Board, id: &u32) -> Trader {
            board
                .agent_grid
                .get(&Trader::dummy(*id))
                .expect("get agent by id")
        }
    } else {
        pub fn get_agent_by_id(board: &mut Board, id: &u32) -> Trader {
            let loc = &board.agent_grid.get_location(&Trader::dummy(*id)).unwrap();
            let traders: Vec<Trader> = board.agent_grid.get_objects(loc).unwrap();
            *traders.into_iter().filter(|trader| trader.id() == *id).collect_vec().first().unwrap()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
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
}
