use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub enum Action {
    North,
    East,
    South,
    West
}

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        match rng.gen_range(0..=3) {
            0 => Action::North,
            1 => Action::East,
            2 => Action::South,
            _ => Action::West
        }
    }
}