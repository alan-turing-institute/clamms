use crate::config::core_config;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::board::Patch;

#[derive(Clone, Copy, Debug)]
pub enum EnvItem {
    Land,
    Bush,
    Resource(Resource),
}

impl Distribution<EnvItem> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnvItem {
        let pick: f32 = rng.gen();
        if pick < core_config().world.FOOD_ABUNDANCE {
            EnvItem::Resource(Resource::Food)
        } else if pick < core_config().world.FOOD_ABUNDANCE + core_config().world.WATER_ABUNDANCE {
            EnvItem::Resource(Resource::Water)
        } else if rng.gen::<f32>() < core_config().world.LAND_PROP {
            EnvItem::Land
        } else {
            EnvItem::Bush
        }
    }
}

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, EnumIter, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum Resource {
    Food,
    Water,
}

impl Resource {
    pub fn texture(&self) -> String {
        match self {
            Resource::Food => "fruit".to_string(),
            Resource::Water => "water".to_string(),
        }
    }

    pub fn to_patch(self, id: u32) -> Patch {
        Patch {
            id,
            env_item: EnvItem::Resource(self),
        }
    }
}
