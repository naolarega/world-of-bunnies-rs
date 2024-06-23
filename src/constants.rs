use crate::utils::RandomNumberGenerator;

#[derive(Debug)]
pub enum Sex {
    Male,
    Female,
}

impl Sex {
    pub fn random_sex(rng: &mut impl RandomNumberGenerator) -> Sex {
        match rng.gen_range(0, 2) {
            0 => Sex::Male,
            1 => Sex::Female,
            unknown_sex => panic!("Unknown sex `{unknown_sex}`"),
        }
    }
}

#[derive(Debug)]
pub enum Color {
    White,
    Brown,
    Black,
    Spotted,
}

impl Color {
    pub fn random_color(rng: &mut impl RandomNumberGenerator) -> Color {
        match rng.gen_range(0, 4) {
            0 => Color::White,
            1 => Color::Brown,
            2 => Color::Black,
            3 => Color::Spotted,
            unknown_color => panic!("Unknown color `{unknown_color}`"),
        }
    }
}

const NAMES_LEN: usize = 40;
const NAMES: [&str; NAMES_LEN] = [
    "Thumper",
    "Flopsy",
    "Cottontail",
    "Binky",
    "Hoppy",
    "Snowball",
    "Nibbles",
    "Pippin",
    "Blossom",
    "Buttercup",
    "Patches",
    "Luna",
    "Daisy",
    "Clover",
    "Hazel",
    "Willow",
    "Pebbles",
    "Mochi",
    "Twinkle",
    "Marshmallow",
    "Caramel",
    "Maple",
    "Tinkerbell",
    "Poppy",
    "Bubbles",
    "Oreo",
    "Muffin",
    "Whiskers",
    "Sprout",
    "Petal",
    "Rusty",
    "Velvet",
    "Sparky",
    "Sunny",
    "Skittles",
    "Honey",
    "Ziggy",
    "Snickers",
    "Fudge",
    "S’mores",
];

pub fn random_name(rng: &mut impl RandomNumberGenerator) -> &'static str {
    NAMES[rng.gen_range(0, NAMES_LEN as u32) as usize]
}
