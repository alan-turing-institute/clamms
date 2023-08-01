use super::trader::Offer;
use crate::config::core_config;
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
    SetOfferWaterFoodLL,
    SetOfferWaterFoodLM,
    SetOfferWaterFoodLH,
    SetOfferWaterFoodML,
    SetOfferWaterFoodMM,
    SetOfferWaterFoodMH,
    SetOfferWaterFoodHL,
    SetOfferWaterFoodHM,
    SetOfferWaterFoodHH,
    SetOfferFoodWaterLL,
    SetOfferFoodWaterLM,
    SetOfferFoodWaterLH,
    SetOfferFoodWaterML,
    SetOfferFoodWaterMM,
    SetOfferFoodWaterMH,
    SetOfferFoodWaterHL,
    SetOfferFoodWaterHM,
    SetOfferFoodWaterHH,
    SetOfferTrivial,
    // SetOfferFoodMoney(Propensity, Propensity),
    // SetOfferMoneyFood(Propensity, Propensity),
    // SetOfferWaterMoney(Propensity, Propensity),
    // SetOfferMoneyWater(Propensity, Propensity),
}

impl Action {
    pub fn parse_offer(&self) -> Option<Offer> {
        let l = core_config().trade.LOW_LOT_SIZE;
        let m = core_config().trade.MEDIUM_LOT_SIZE;
        let h = core_config().trade.HIGH_LOT_SIZE;
        match self {
            Action::ToFood => None,
            Action::ToWater => None,
            Action::ToAgent => None,
            Action::Stationary => None,
            Action::SetOfferWaterFoodLL => Some(Offer::new(l, -l)),
            Action::SetOfferWaterFoodLM => Some(Offer::new(l, -m)),
            Action::SetOfferWaterFoodLH => Some(Offer::new(l, -h)),
            Action::SetOfferWaterFoodML => Some(Offer::new(m, -l)),
            Action::SetOfferWaterFoodMM => Some(Offer::new(m, -m)),
            Action::SetOfferWaterFoodMH => Some(Offer::new(m, -h)),
            Action::SetOfferWaterFoodHL => Some(Offer::new(h, -l)),
            Action::SetOfferWaterFoodHM => Some(Offer::new(h, -m)),
            Action::SetOfferWaterFoodHH => Some(Offer::new(h, -h)),
            Action::SetOfferFoodWaterLL => Some(Offer::new(-l, l)),
            Action::SetOfferFoodWaterLM => Some(Offer::new(-l, m)),
            Action::SetOfferFoodWaterLH => Some(Offer::new(-l, h)),
            Action::SetOfferFoodWaterML => Some(Offer::new(-m, l)),
            Action::SetOfferFoodWaterMM => Some(Offer::new(-m, m)),
            Action::SetOfferFoodWaterMH => Some(Offer::new(-m, h)),
            Action::SetOfferFoodWaterHL => Some(Offer::new(-h, l)),
            Action::SetOfferFoodWaterHM => Some(Offer::new(-h, m)),
            Action::SetOfferFoodWaterHH => Some(Offer::new(-h, h)),
            Action::SetOfferTrivial => Some(Offer::new(0, 0)),
        }
    }
}

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        match rng.gen_range(0..=21) {
            0 => Action::ToFood,
            1 => Action::ToWater,
            2 => Action::Stationary,
            3 => Action::SetOfferWaterFoodLL,
            4 => Action::SetOfferWaterFoodLM,
            5 => Action::SetOfferWaterFoodLH,
            6 => Action::SetOfferWaterFoodML,
            7 => Action::SetOfferWaterFoodMM,
            8 => Action::SetOfferWaterFoodMH,
            9 => Action::SetOfferWaterFoodHL,
            10 => Action::SetOfferWaterFoodHM,
            11 => Action::SetOfferWaterFoodHH,
            12 => Action::SetOfferFoodWaterLL,
            13 => Action::SetOfferFoodWaterLM,
            14 => Action::SetOfferFoodWaterLH,
            15 => Action::SetOfferFoodWaterML,
            16 => Action::SetOfferFoodWaterMM,
            17 => Action::SetOfferFoodWaterMH,
            18 => Action::SetOfferFoodWaterHL,
            19 => Action::SetOfferFoodWaterHM,
            20 => Action::SetOfferFoodWaterHH,
            _ => Action::SetOfferTrivial,
        }
    }
}
