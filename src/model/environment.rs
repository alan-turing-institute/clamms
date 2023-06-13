use std::path::Display;

use rand::thread_rng;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// use crate::config::{CORE_CONFIG.world. FOOD_ABUNDANCE, WATER_ABUNDANCE};
use crate::config::core_config;

#[derive(Clone, Copy, Debug)]
pub enum EnvItem {
    Land,
    Resource(Resource),
}

impl Distribution<EnvItem> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnvItem {
        let mut rng = thread_rng();
        let pick: f32 = rng.gen();
        if pick < core_config().world.FOOD_ABUNDANCE {
            EnvItem::Resource(Resource::Food)
        } else if pick < core_config().world.FOOD_ABUNDANCE + core_config().world.WATER_ABUNDANCE {
            EnvItem::Resource(Resource::Water)
        } else {
            EnvItem::Land
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Resource {
    Food,
    Water,
}

impl Resource {
    pub fn texture(&self) -> String {
        match self {
            Resource::Food => "tree".to_string(),
            Resource::Water => "water".to_string(),
        }
    }
}
