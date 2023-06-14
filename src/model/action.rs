use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, Hash, Eq)]
pub enum Action {
    ToFood,
    ToWater,
    Stationary,
}
