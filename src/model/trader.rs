use krabmaga::engine::agent::Agent;

use super::{inventory::Inventory, routing::{Position, Router}, forager::Forager, environment::Resource};


#[derive(Clone, Copy)]
pub struct Trader { 
    forager: Forager
}

impl Trader {
    pub fn new(forager: Forager) -> Self {
        Trader{ forager }
    }
}

// pub struct Offer<Vec<Resource>> {

// }

// pub trait Trade {
//     fn offer(&self, resource: Resource) -> u32;
//     fn is_viable(&self, resource: Resource) -> u32;
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

impl Inventory for Trader {
    fn count(&self, resource: &Resource) -> i32 {
        self.forager.count(resource)
    }

    fn acquire(&mut self, resource: &Resource, quantity: i32) -> () {
        self.forager.acquire(resource, quantity)
    }
}