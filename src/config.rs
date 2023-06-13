// global state
pub type ResourceAbundance = f32;
pub const FOOD_ABUNDANCE: ResourceAbundance = 0.1;
pub const WATER_ABUNDANCE: ResourceAbundance = 0.1;

pub const TREE_PROB: f32 = 0.1;
pub const SWEET_PROB: f32 = 0.01;

pub const INIT_FOOD: i32 = 0;
pub const INIT_WATER: i32 = 0;

pub const FOOD_ACQUIRE_RATE: i32 = 10;
pub const WATER_ACQUIRE_RATE: i32 = 10;

pub const FOOD_CONSUME_RATE: u32 = 1;
pub const WATER_CONSUME_RATE: u32 = 1;

pub const FOOD_MAX_INVENTORY: i32 = 100;
pub const WATER_MAX_INVENTORY: i32 = 100;

// TODO: add optional rng seed

// pub struct Config {
//     sim_init: SimInit
// }
