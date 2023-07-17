use super::agent_api::AgentAPI;
use super::board::AgentOffer;
use super::routing::step_distance;
use krabmaga::engine::{agent::Agent, location::Int2D};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::{Hash, Hasher};
// use std::error::Error;
use super::{
    environment::Resource,
    forager::Forager,
    inventory::Inventory,
    routing::{Position, Router},
};
use crate::{config::core_config, model::board::Board};

#[derive(Clone, Copy)]
pub struct Trader {
    pub forager: Forager,
}

impl Display for Trader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}; Loc: ({}, {}); Food: {} Water: {}",
            self.id(),
            self.forager.pos.x,
            self.forager.pos.y,
            self.forager.count(&Resource::Food),
            self.forager.count(&Resource::Water)
        )
    }
}

impl Trader {
    pub fn new(forager: Forager) -> Self {
        Trader { forager }
    }

    pub fn forager(&self) -> &Forager {
        &self.forager
    }

    pub fn id(&self) -> u32 {
        self.forager.id()
    }

    /// Dummy trader for matching just on ID.
    pub fn dummy(id: u32) -> Self {
        Trader {
            forager: Forager::dummy(id),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct Offer(i32, i32);

// #[derive(Error, Debug)]
// pub enum OfferError {
//     /// Incorrect sign on offer value.
//     #[error("Incorrect sign on value of offered lots: {0}.")]
//     IncorrectSign(i32),
// }

impl Offer {
    pub fn new(lots_food: i32, lots_water: i32) -> Self {
        if lots_food > 0 && lots_water > 0 {
            panic!();
        }
        if lots_food < 0 && lots_water < 0 {
            panic!();
        }
        Offer(lots_food, lots_water)
    }

    fn food_delta(&self) -> i32 {
        self.0
    }

    fn water_delta(&self) -> i32 {
        self.1
    }

    /// Number of lots offered. Will always be non-positive.
    fn offered_lots(&self) -> i32 {
        std::cmp::min(self.0, self.1)
    }

    /// Number of lots demanded. Will always be non-negative
    fn demanded_lots(&self) -> i32 {
        std::cmp::max(self.0, self.1)
    }

    pub fn is_trivial(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }

    /// Adjust this offer by one lot offered & demanded.
    fn adjust_by_one(&mut self, food_is_offered: bool) {
        if food_is_offered {
            self.0 -= 1;
            self.1 += 1;
        } else {
            self.0 += 1;
            self.1 -= 1;
        }
    }

    /// Determines whether this offer is matched by another offer.
    pub fn matched(&self, offer: &Offer) -> bool {
        std::cmp::max(self.0 + offer.0, self.1 + offer.1) <= 0
    }

    pub fn invert(&self) -> Offer {
        Offer(self.1, self.0)
    }
}

pub trait Trade {
    /// Gets this trader's offer.
    fn offer(&self) -> Offer;
    /// Decides whether this trader is prepared to raise the given current offer.
    fn will_raise_offer(
        &self,
        current_offer: &Offer,
        offered_count: i32,
        other_count: i32,
        offered_lot_size: u32,
        other_lot_size: u32,
    ) -> bool;
    /// Applies their offer during trading.
    fn apply_offer(&mut self);
}

impl Trade for Trader {
    /// Makes an offer, given the agent's current inventory.
    fn offer(&self) -> Offer {
        let mut current_offer = Offer::new(0, 0);

        // Offer the resource with maximum supply and demand the one with minimum supply.
        // Find the maximum trade of food such that the food inventory will
        // be greater than the water inventory even after the trade.
        let count_food = self.count(&Resource::Food);
        let count_water = self.count(&Resource::Water);
        while self.will_raise_offer(
            &current_offer,
            count_food,
            count_water,
            core_config().agent.FOOD_LOT_SIZE,
            core_config().agent.WATER_LOT_SIZE,
        ) && (count_food + current_offer.0) > core_config().trade.FOOD_MIN_INVENTORY_LEVEL
        {
            current_offer.adjust_by_one(true);
        }
        if !current_offer.is_trivial() {
            return current_offer;
        }

        while self.will_raise_offer(
            &current_offer,
            count_water,
            count_food,
            core_config().agent.WATER_LOT_SIZE,
            core_config().agent.FOOD_LOT_SIZE,
        ) && (count_water + current_offer.1) > core_config().trade.WATER_MIN_INVENTORY_LEVEL
        {
            current_offer.adjust_by_one(false);
        }
        current_offer
    }

    /// Predicate to decide whether a higher offer will be made.
    fn will_raise_offer(
        &self,
        current_offer: &Offer,
        offered_count: i32,
        demanded_count: i32,
        offered_lot_size: u32,
        demanded_lot_size: u32,
    ) -> bool {
        let offered_lots = current_offer.offered_lots();
        if offered_lots.abs() as u32 >= core_config().agent.MAX_TRADE_LOTS {
            return false;
        }
        // Naively, the offer is max when the inventory of the offered resource would remain larger than that of
        // the demanded resource even after the exchange of an additional lot (in each direction).
        // In general, other resource characteristics (consumption rate, acquisition rate, etc.) should
        // also be taken into account.
        let demanded_lots = current_offer.demanded_lots();
        offered_count + ((offered_lots - 1) * (offered_lot_size as i32))
            > demanded_count + ((demanded_lots + 1) * (demanded_lot_size as i32))
    }

    fn apply_offer(&mut self) {
        let offer = self.offer();
        // Settle food inventory.
        self.acquire(&Resource::Food, offer.food_delta());
        // Settle water inventory.
        self.acquire(&Resource::Water, offer.water_delta());
    }
}

impl Agent for Trader {
    fn step(&mut self, state: &mut dyn krabmaga::engine::state::State) {
        let board = state.as_any_mut().downcast_mut::<Board>().unwrap();
        // Borrow traders snapshot captured at start of current board step in before_step
        let traders = &board.current_traders;
        if (board.step > 0) & board.has_trading {
            // Execute trade if available.
            if !self.offer().is_trivial() {
                if !board.traded.contains_key(&self.id()) {
                    let offer = self.offer();
                    for counterparty in traders {
                        let counterparty_id = counterparty.id();
                        // If already traded, continue
                        if board.traded.contains_key(&counterparty_id) {
                            continue;
                        }
                        // If not self AND offer is matched AND agents are close enough, perform trade
                        if counterparty_id != self.id()
                            && counterparty.offer().matched(&offer)
                            && (step_distance(&self.forager.pos, &counterparty.forager.pos)
                                < core_config().trade.MAX_TRADE_DISTANCE)
                        {
                            // Print trade when vverbose
                            if core_config().simulation.VERBOSITY > 1 {
                                println!("Trade between: {} and {}", self, counterparty);
                            }
                            // Add trade to lookup of which agents have traded
                            board.traded.insert(
                                self.id(),
                                Some(AgentOffer::new(counterparty.id(), &offer)),
                            );
                            board.traded.insert(
                                counterparty.id(),
                                Some(AgentOffer::new(self.id(), &offer.invert())),
                            );

                            // Apply offer to inventory, counterparty will do corresponding call
                            // during their update
                            self.apply_offer();

                            // Break - trade has occurred with only single trade currently implemented
                            break;
                        }
                    }
                    // If no trade possible and not traded, set to None
                    board.traded.entry(self.id()).or_insert(None);
                } else if let Some(&Some(_)) = board.traded.get(&self.id()) {
                    // Apply offer previously initiated by a counterparty during their agent step
                    self.apply_offer();
                }
            } else {
                // Offer trivial, set to None
                board.traded.insert(self.id(), None);
            }
        }

        // Trade has occurred before agent choosee next action
        self.forager.step(state)
    }
}

// impl Policy for Trader {
//     fn choose_action(&self, agent_state: &AgentState) -> Action {
//         // Forage unless making a non-trivial offer.
//         if self.offer().is_trivial() {
//             return self.forager.choose_action(agent_state);
//         }
//         // If another agent is closer than any resources, move towards them.
//         if let Some(min_steps_to_trader) = agent_state.min_steps_to_trader {
//             if agent_state.min_steps_to_food.is_none() || agent_state.min_steps_to_water.is_none() {
//                 return Action::ToAgent;
//             }
//             if min_steps_to_trader
//                 < std::cmp::min(
//                     agent_state.min_steps_to_food.unwrap(),
//                     agent_state.min_steps_to_water.unwrap(),
//                 )
//             {
//                 return Action::ToAgent;
//             }
//         }
//         //  Otherwise forage.
//         self.forager.choose_action(agent_state)
//     }
// }

impl Position for Trader {
    fn get_position(&self) -> Int2D {
        self.forager.pos.to_owned()
    }
}

impl Router for Trader {}

impl Inventory for Trader {
    fn count(&self, resource: &Resource) -> i32 {
        self.forager.count(resource)
    }

    fn acquire(&mut self, resource: &Resource, quantity: i32) {
        self.forager.acquire(resource, quantity)
    }
}

impl Eq for Trader {}

impl PartialEq for Trader {
    fn eq(&self, other: &Trader) -> bool {
        self.id() == other.id()
    }
}

impl Hash for Trader {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::model::init;

    use super::*;

    #[test]
    fn test_matched() {
        // This is an offer of *at most* 2 lots of food for *at least* 3 lots of water.
        let offer = Offer::new(-2, 3);

        assert!(offer.matched(&Offer::new(2, -3)));
        assert!(offer.matched(&Offer::new(2, -5)));
        assert!(offer.matched(&Offer::new(1, -3)));
        assert!(offer.matched(&Offer::new(1, -4)));
        assert!(offer.matched(&Offer::new(0, -3)));

        assert!(!offer.matched(&Offer::new(3, -3)));
        assert!(!offer.matched(&Offer::new(2, -2)));
        assert!(!offer.matched(&Offer::new(2, -1)));
    }

    #[test]
    fn test_trader_offer() {
        // Test init has FOOD_LOT_SIZE and WATER_LOT_SIZE equal to 1 and MIN_INVENTORY_LEVEL = 0
        init();
        let pos = Int2D { x: 0, y: 0 };
        assert_eq!(
            Trader::new(Forager::new(0, pos, 8, 5)).offer(),
            Offer::new(-1, 1)
        );
        assert_eq!(
            Trader::new(Forager::new(0, pos, 7, 5)).offer(),
            Offer::new(0, 0)
        );
        assert_eq!(
            Trader::new(Forager::new(0, pos, -8, -5)).offer(),
            Offer::new(0, 0)
        );
        assert_eq!(
            Trader::new(Forager::new(0, pos, 2, -1)).offer(),
            Offer::new(-1, 1)
        );
        assert_eq!(
            Trader::new(Forager::new(0, pos, 1, -2)).offer(),
            Offer::new(-1, 1)
        );
        assert_eq!(
            Trader::new(Forager::new(0, pos, -2, 1)).offer(),
            Offer::new(1, -1)
        );
        assert_eq!(
            Trader::new(Forager::new(0, pos, 0, -1)).offer(),
            Offer::new(0, 0)
        );
    }
}
