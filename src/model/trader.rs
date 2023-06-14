use krabmaga::engine::{agent::Agent, location::Int2D};
use std::hash::{Hash, Hasher};

use super::{inventory::Inventory, routing::{Position, Router}, forager::Forager, environment::Resource};


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

// pub type Offer = (i8, i8);

pub struct Offer(i8, i8);

impl Offer {

    fn new(lots_food: i8, lots_water: i8) -> Self {
        if lots_food > 0 && lots_water > 0 {
            panic!();
        }
        if lots_food < 0 && lots_water < 0 {
            panic!();
        }
        Offer(lots_food, lots_water)
    }

    fn delta(&self, resource: Resource) -> i8 {
        match resource {
            Resource::Food => self.0,
            Resource::Water => self.1
        }
    }

    /// Determines whether this offer is matched by another offer.
    fn matched(&self, offer: Offer) -> bool {
        std::cmp::max(self.0 + offer.0, self.1 + offer.1) <= 0
    }
}

pub trait Trade {
    fn offer(&self) -> Offer;
}


// impl Trade for Trader {
//     fn offer(&self, resource: Resource) -> u32 {
//         todo!()
//     }

//     fn demand(&self, resource: Resource) -> u32 {
//         todo!()
//     }
// }

// impl Agent for Trader {
    
//     fn step(&mut self, state: &mut dyn krabmaga::engine::state::State) {
//         todo!()
//     }
// }

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
    use crate::{model::{forager::Forager, trader::Trader}, config::core_config};

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