use krabmaga::{engine::{agent::Agent, location::Int2D}};
use krabmaga::cfg_if::cfg_if;
use strum_macros::Display;
use std::{hash::{Hash, Hasher}};
// use std::error::Error;
use crate::{config::core_config, model::board::Board};
use super::{inventory::Inventory, routing::{Position, Router, get_trader_locations, get_resource_locations, get_traders}, forager::Forager, environment::Resource, policy::Policy, agent_state::AgentState, action::Action};
use krabmaga::utils;

use crate::engine::fields::grid_option::GridOption;


#[derive(Clone, Copy)]
pub struct Trader { 
    pub forager: Forager
}

impl Trader {
    pub fn new(forager: Forager) -> Self {
        Trader{ forager }
    }

    pub fn forager(&self) -> &Forager {
        &self.forager
    }

    pub fn id(&self) -> u32 {
        self.forager.id()
    }

    /// Dummy trader for matching just on ID.
    pub fn dummy(id: u32) -> Self {
        Trader {forager: Forager::dummy(id)}
    }
}

#[derive(Debug)]
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
    fn matched(&self, offer: &Offer) -> bool {
        std::cmp::max(self.0 + offer.0, self.1 + offer.1) <= 0
    }

    fn invert(&self) -> Offer {
        Offer(self.1, self.0)
    }
}

pub trait Trade {

    fn offer(&self) -> Offer;
    fn will_raise_offer(&self, current_offer: &Offer, offered_count: i32, other_count: i32, offered_lot_size: u32, other_lot_size: u32) -> bool;
    fn settle_trade(&mut self, counterparty: &mut Trader);
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

    fn settle_trade(&mut self, counterparty: &mut Trader) {

        // Settle according to the offer of *this* trader (not the counterparty's offer).
        let offer = self.offer();
        if !offer.matched(&counterparty.offer()) {
            panic!("Trade can't be settled!");
        }

        // Settle food inventory for both agents.
        self.acquire(&Resource::Food, offer.food_delta());
        counterparty.acquire(&Resource::Food, -1 * offer.food_delta());

        // Settle water inventory for both agents.
        self.acquire(&Resource::Water, offer.water_delta());
        counterparty.acquire(&Resource::Water, -1 * offer.water_delta());

        println!("***** TRADE SETTLED FOR {:?} BETWEEN TRADER {} and TRADER {} *****", offer, self.id(), counterparty.id())
    }
}

fn settle_trade_on_counterparty(mut counterparty: Trader, offer: &Offer) -> Trader {

    if !offer.matched(&counterparty.offer()) {
        // Do nothing.
        return counterparty
    }

    // Settle inventories.
    counterparty.acquire(&Resource::Food, -1 * offer.food_delta());
    counterparty.acquire(&Resource::Water, -1 * offer.water_delta());    

    println!("***** TRADE SETTLED FOR {:?} WITH TRADER {} *****", offer, counterparty.id());

    counterparty
}

impl Agent for Trader {
    
    fn step(&mut self, state: &mut dyn krabmaga::engine::state::State) {

        let state = state.as_any_mut().downcast_mut::<Board>().unwrap();
        let agent_state = self.forager.agent_state(state);

        // select action from policy
        // let action = self.choose_action(&agent_state);

        // // route agent based on action (delegated to forager, including move towards trader)
        // let route = match action {
        //     Action::ToAgent => self.try_move_towards_agent(state, None),
        //     _ => None,
        // };

        // Execute trade if available.
        if !self.offer().is_trivial() {

            // Get traders as mutable...(hack):
            // TODO: only insider trading horizon.
            cfg_if! {
                if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
                    let offer = self.offer();
                    state.agent_grid.apply_to_all_values(|_, trader| {
                        if trader.offer().matched(&offer) {
                            // TODO: consider picking one trader at random instead
                            // of *this* trader. (No real advantage though.)
                            settle_trade_on_counterparty(*trader, &offer);
                            // Now settle the trade on *this* Trader's inventory.
                            settle_trade_on_counterparty(*self, &offer.invert());
                            return Some(*trader)
                        }
                        None
                    }, GridOption::READ)

                    // for trader in state.trader_grid.loc2objs.values_mut().into_iter() {

                    //     if trader.offer().matched(self.offer()) {
                    //         // TODO: consider picking one trader at random instead
                    //         // of *this* trader. (No real advantage though.)
                    //         self.settle_trade(trader);
                    //     }
                    // }
    
                } else {
                    for ref_cell in state.trader_grid.locs.iter_mut() {
                        for x in ref_cell.borrow_mut().iter_mut() {
                            for trader in x.iter_mut() {
                                if trader.offer().matched(&self.offer()) {
                                    // TODO: consider picking one trader at random instead
                                    // of *this* trader. (No real advantage though.)
                                    self.settle_trade(trader);
                                }
                            }
                        }
                    }
                }    
            }
        }

        // Always finish by delegating to the wrapped forager:
        self.forager.step(state)
    }
}

impl Policy for Trader {

    fn choose_action(&self, agent_state: &AgentState) -> Action {
        
        // Forage unless making a non-trivial offer.
        if self.offer().is_trivial() {
            return self.forager.choose_action(agent_state)
        }
        // If another agent is closer than any resources, move towards them.
        if let Some(min_steps_to_trader) = agent_state.min_steps_to_trader {
            if agent_state.min_steps_to_food.is_none() || agent_state.min_steps_to_water.is_none() {
                return Action::ToAgent
            }
            if min_steps_to_trader < std::cmp::min(agent_state.min_steps_to_food.unwrap(), agent_state.min_steps_to_water.unwrap()) {
                return Action::ToAgent
            }
        }
        //  Otherwise forage.
        self.forager.choose_action(agent_state)
    }
}


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

        assert!(offer.matched(&Offer::new(2, -3)));
        assert!(offer.matched(&Offer::new(2, -5)));
        assert!(offer.matched(&Offer::new(1, -3)));
        assert!(offer.matched(&Offer::new(1, -4)));
        assert!(offer.matched(&Offer::new(0, -3)));
        
        assert!(!offer.matched(&Offer::new(3, -3)));
        assert!(!offer.matched(&Offer::new(2, -2)));
        assert!(!offer.matched(&Offer::new(2, -1)));
    }
}