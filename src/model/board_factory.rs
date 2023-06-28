use super::environment::Resource;
use krabmaga::engine::location::Int2D;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;

/// Clamms version of Int2D with trait implementations and conversion to Int2D
#[derive(Clone, Copy, Serialize, Deserialize, Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct ClammsInt2D {
    pub x: i32,
    pub y: i32,
}

impl From<ClammsInt2D> for Int2D {
    fn from(value: ClammsInt2D) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
///
pub fn read_resource_locations(input: &str) -> BTreeMap<Resource, Vec<Int2D>> {
    serde_json::from_str::<BTreeMap<Resource, Vec<ClammsInt2D>>>(input)
        .unwrap()
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().map(Int2D::from).collect()))
        .collect()
}

pub fn example_board(dim: (u16, u16)) -> BTreeMap<Resource, Vec<ClammsInt2D>> {
    let mut map = BTreeMap::new();
    map.insert(Resource::Food, vec![]);
    map.insert(Resource::Water, vec![]);
    for i in 1..7 {
        for j in 1..7 {
            let v = map.get_mut(&Resource::Food).unwrap();
            v.push(ClammsInt2D { x: i, y: j });
        }
    }
    let mut rng = StdRng::seed_from_u64(1);
    let mut river_width = 0i32;
    for j in 1..dim.1 {
        river_width += rng.gen_range(-2..=2);
        for i in (dim.0 / 2 - 2)..=(dim.0 / 2 + 2) {
            let v = map.get_mut(&Resource::Water).unwrap();
            v.push(ClammsInt2D {
                x: (i as i32 - 2 + river_width),
                y: j.into(),
            });
        }
    }
    map
}

#[cfg(test)]
mod tests {}
