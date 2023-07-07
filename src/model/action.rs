use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Default, Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, Hash, Eq, PartialOrd, Ord,
)]
pub enum Propensity {
    #[default]
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, Hash, Eq)]
pub enum Action {
    ToFood,
    ToWater,
    ToAgent,
    Stationary,
    SetOfferFoodWater(Propensity, Propensity),
    SetOfferWaterFood(Propensity, Propensity),
    SetOfferTrivial,
    // SetOfferFoodMoney(Propensity, Propensity),
    // SetOfferMoneyFood(Propensity, Propensity),
    // SetOfferWaterMoney(Propensity, Propensity),
    // SetOfferMoneyWater(Propensity, Propensity),
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
