use krabmaga::engine::{agent::Agent, location::Int2D};
use std::{hash::{Hash, Hasher}};
// use std::error::Error;
use crate::config::core_config;
use super::{inventory::Inventory, routing::{Position, Router, get_trader_locations, get_resource_locations}, forager::Forager, environment::Resource, policy::Policy, agent_state::AgentState, action::Action};


#[derive(Clone, Copy)]
pub struct Trader { 
    forager: Forager
}

impl Trader {
    pub fn new(forager: Forager) -> Self {
        Trader{ forager }
    }

    pub fn id(&self) -> u32 {
        self.forager.id()
    }
}

pub struct Offer(i32, i32);

// #[derive(Error, Debug)]
// pub enum OfferError {
//     /// Incorrect sign on offer value.
//     #[error("Incorrect sign on value of offered lots: {0}.")]
//     IncorrectSign(i32),
// }

impl Offer {

    fn new(lots_food: i32, lots_water: i32) -> Self {
        if lots_food > 0 && lots_water > 0 {
            panic!();
        }
        if lots_food < 0 && lots_water < 0 {
            panic!();
        }
        Offer(lots_food, lots_water)
    }

    /// Number of lots offered. Will always be non-positive.
    fn offered_lots(&self) -> i32 {
        std::cmp::min(self.0, self.1)
    }

    /// Number of lots demanded. Will always be non-negative
    fn demanded_lots(&self) -> i32 {
        std::cmp::max(self.0, self.1)
    }    

    fn is_trivial(&self) -> bool {
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
    fn matched(&self, offer: Offer) -> bool {
        std::cmp::max(self.0 + offer.0, self.1 + offer.1) <= 0
    }
}

pub trait Trade {

    fn offer(&self) -> Offer;
    fn will_raise_offer(&self, current_offer: &Offer, offered_count: i32, other_count: i32, offered_lot_size: u32, other_lot_size: u32) -> bool;
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
        while self.will_raise_offer(&current_offer, count_food, count_water, core_config().agent.FOOD_LOT_SIZE, core_config().agent.WATER_LOT_SIZE) {
            current_offer.adjust_by_one(true);
        }
        if !current_offer.is_trivial() {
            return current_offer
        }

        while self.will_raise_offer(&current_offer, count_water, count_food, core_config().agent.WATER_LOT_SIZE, core_config().agent.FOOD_LOT_SIZE) {
            current_offer.adjust_by_one(false);
        }
        current_offer
    }

    /// Predicate to decide whether a higher offer will be made.
    fn will_raise_offer(&self, current_offer: &Offer, offered_count: i32, demanded_count: i32, offered_lot_size: u32, demanded_lot_size: u32) -> bool {
        let offered_lots = current_offer.offered_lots();
        if offered_lots.abs() as u32 >= core_config().agent.MAX_TRADE_LOTS {
            return false
        }
        // Naively, the offer is max when the inventory of the offered resource would remain larger than that of
        // the demanded resource even after the exchange of an additional lot (in each direction).
        // In general, other resource characteristics (consumption rate, acquisition rate, etc.) should
        // also be taken into account.
        let demanded_lots = current_offer.demanded_lots();
        offered_count + ((offered_lots - 1) * (offered_lot_size as i32)) > demanded_count + ((demanded_lots + 1) * (demanded_lot_size as i32))
    }
}

// impl Agent for Trader {
    
//     fn step(&mut self, state: &mut dyn krabmaga::engine::state::State) {
//         todo!()
//     }
// }

impl Policy for Trader {

    fn chose_action(&self, agent_state: &AgentState) -> Action {
        panic!("Use choose_action method instead!");
    }
    
    fn choose_action(&self, state: &dyn krabmaga::engine::state::State) -> Action {
        
        // If another agent is closer than any resources, move towards them.
        let min_steps_to_food = self.min_steps_to(get_resource_locations(&Resource::Food, state)).expect("Food source must exist");
        let min_steps_to_water = self.min_steps_to(get_resource_locations(&Resource::Water, state)).expect("Water source must exist");
        
        if let Some(min_steps_to_trader) = self.min_steps_to(get_trader_locations(state)) {
            if min_steps_to_trader < std::cmp::min(min_steps_to_food, min_steps_to_water) {
                return Action::ToAgent
            }
        }
        self.forager.choose_action(state)
    }
}


impl Position for Trader {
    fn get_position(&self) -> Int2D {
        self.forager.pos.to_owned()
    }
}

impl Inventory for Trader {
    fn count(&self, resource: &Resource) -> i32 {
        self.forager.count(resource)
    }

    fn acquire(&mut self, resource: &Resource, quantity: i32) -> () {
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
    use super::*;

    #[test]
    fn test_matched() {

        // This is an offer of *at most* 2 lots of food for *at least* 3 lots of water.
        let offer = Offer::new(-2, 3);

        assert!(offer.matched(Offer::new(2, -3)));
        assert!(offer.matched(Offer::new(2, -5)));
        assert!(offer.matched(Offer::new(1, -3)));
        assert!(offer.matched(Offer::new(1, -4)));
        assert!(offer.matched(Offer::new(0, -3)));
        
        assert!(!offer.matched(Offer::new(3, -3)));
        assert!(!offer.matched(Offer::new(2, -2)));
        assert!(!offer.matched(Offer::new(2, -1)));
    }
}