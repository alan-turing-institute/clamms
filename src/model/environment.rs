use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use strum_macros::EnumIter;

use crate::config::{FOOD_ABUNDANCE, WATER_ABUNDANCE};

use super::board::Patch;

#[derive(Clone, Copy, Debug)]
pub enum EnvItem {
    Land,
    Resource(Resource),
}

impl Distribution<EnvItem> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnvItem {
        let pick: f32 = rng.gen();
        if pick < FOOD_ABUNDANCE {
            EnvItem::Resource(Resource::Food)
        } else if pick < FOOD_ABUNDANCE + WATER_ABUNDANCE {
            EnvItem::Resource(Resource::Water)
        } else {
            EnvItem::Land
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, EnumIter)]
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

    pub fn to_patch(self, id: u32) -> Patch {
        Patch {
            id,
            env_item: EnvItem::Resource(self),
        }
    }
}
