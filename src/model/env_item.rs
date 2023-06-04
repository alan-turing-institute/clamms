use rand::thread_rng;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::config::{TREE_PROB, SWEET_PROB};

#[derive(Clone, Copy, Debug)]
pub enum EnvItem {
    Tree,
    Land,
    Sweet
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