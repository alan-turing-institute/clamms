use rand::thread_rng;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::config::{SWEET_PROB, TREE_PROB};

#[derive(Clone, Copy, Debug)]
pub enum EnvItem {
    Tree,
    Land,
    Sweet,
    Resource(Resource),
}

impl Distribution<EnvItem> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnvItem {
        let mut rng = thread_rng();
        let pick: f32 = rng.gen();
        if pick < SWEET_PROB {
            EnvItem::Sweet
        } else if pick < SWEET_PROB + TREE_PROB {
            EnvItem::Tree
        } else {
            EnvItem::Land
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Resource {}
