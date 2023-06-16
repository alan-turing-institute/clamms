use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, Hash, Eq)]
pub enum Action {
    ToFood,
    ToWater,
    ToAgent,
    Stationary,
}

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        match rng.gen_range(0..=2) {
            0 => Action::ToFood,
            1 => Action::ToWater,
            _ => Action::Stationary,
        }
    }
}
