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

// pub struct Offer<Vec<Resource>> {

// }

// pub trait Trade {
//     fn offer(&self, resource: Resource) -> u32;
//     fn is_viable(&self, resource: Resource) -> bool;
// }


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
