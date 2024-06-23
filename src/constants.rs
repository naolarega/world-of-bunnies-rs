use crate::utils::{simple_rng::SimpleRNG, RandomNumberGenerator};

pub enum Sex {
    Male,
    Female,
}

impl Sex {
    pub fn random_sex(rng: &mut impl RandomNumberGenerator) -> Sex {
        match rng.gen_range(0, 1) {
            0 => Sex::Male,
            1 => Sex::Female,
            unknown_sex => panic!("Unknown sex `{unknown_sex}`"),
        }
    }
}

pub enum Color {
    White,
    Brown,
    Black,
    Spotted,
}

impl Color {
    pub fn random_color(rng: &mut impl RandomNumberGenerator) -> Color {
        match rng.gen_range(0, 3) {
            0 => Color::White,
            1 => Color::Brown,
            2 => Color::Black,
            3 => Color::Spotted,
            unknown_color => panic!("Unknown color `{unknown_color}`"),
        }
    }
}

const NAMES: [&str; 1] = ["John"];

pub fn random_name(rng: &mut impl RandomNumberGenerator) -> &'static str {
    NAMES[0]
}
