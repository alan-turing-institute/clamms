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

trait Construct {
    fn generate(&self) -> BTreeMap<Resource, Vec<ClammsInt2D>>;
}

struct CornerBoard {
    water_loc: ClammsInt2D,
    food_loc: ClammsInt2D,
    water_size: i32,
    food_size: i32,
    dim: (usize, usize),
}

impl Construct for CornerBoard {
    fn generate(&self) -> BTreeMap<Resource, Vec<ClammsInt2D>> {
        let mut map = BTreeMap::new();
        map.insert(Resource::Food, vec![]);
        map.insert(Resource::Water, vec![]);
        // Set water locations
        assert!((self.water_loc.x + self.water_size) < self.dim.0.try_into().unwrap());
        assert!((self.water_loc.y + self.water_size) < self.dim.1.try_into().unwrap());
        for i in self.water_loc.x..self.water_size {
            for j in self.water_loc.y..(self.water_loc.y + self.water_size) {
                let v = map.get_mut(&Resource::Water).unwrap();
                v.push(ClammsInt2D { x: i, y: j });
            }
        }
        // Set food locations
        assert!((self.food_loc.x + self.food_size) < self.dim.0.try_into().unwrap());
        assert!((self.food_loc.y + self.food_size) < self.dim.1.try_into().unwrap());
        for i in self.food_loc.x..(self.food_loc.x + self.food_size) {
            for j in self.food_loc.y..(self.food_loc.y + self.food_size) {
                let v = map.get_mut(&Resource::Food).unwrap();
                v.push(ClammsInt2D { x: i, y: j });
            }
        }
        map
    }
}

struct RiverBoard {
    _water_loc: ClammsInt2D,
    food_loc: ClammsInt2D,
    water_size: i32,
    food_size: i32,
    dim: (usize, usize),
}

impl Construct for RiverBoard {
    fn generate(&self) -> BTreeMap<Resource, Vec<ClammsInt2D>> {
        let mut map = BTreeMap::new();
        map.insert(Resource::Food, vec![]);
        map.insert(Resource::Water, vec![]);
        // Set food locations
        assert!((self.food_loc.x + self.food_size) < self.dim.0.try_into().unwrap());
        assert!((self.food_loc.y + self.food_size) < self.dim.1.try_into().unwrap());
        for i in self.food_loc.x..self.food_size {
            for j in self.food_loc.y..(self.food_loc.y + self.food_size) {
                let v = map.get_mut(&Resource::Food).unwrap();
                v.push(ClammsInt2D { x: i, y: j });
            }
        }
        // Set water locations
        let mut rng = StdRng::seed_from_u64(1);
        let mut river_width = 0i32;
        for j in 1..self.dim.1 {
            river_width += rng.gen_range(-(self.water_size / 2)..=(self.water_size / 2));
            for i in (self.dim.0 / 2 - usize::try_from(self.water_size).unwrap())
                ..=(self.dim.0 / 2 + usize::try_from(self.water_size).unwrap())
            {
                let v = map.get_mut(&Resource::Water).unwrap();
                v.push(ClammsInt2D {
                    x: (i32::try_from(i).unwrap() - i32::try_from(self.dim.0 / 2).unwrap()
                        + river_width),
                    y: i32::try_from(j).unwrap(),
                });
            }
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corner_board() {
        let dim = (10usize, 10usize);
        let corner_board = CornerBoard {
            water_loc: ClammsInt2D { x: 0, y: 0 },
            food_loc: ClammsInt2D {
                x: i32::try_from(dim.0).unwrap() - 3,
                y: i32::try_from(dim.1).unwrap() - 3,
            },
            water_size: 2,
            food_size: 2,
            dim,
        };
        println!("{:?}", corner_board.generate());
    }
    #[test]
    fn test_river_board() {
        todo!()
    }
}
