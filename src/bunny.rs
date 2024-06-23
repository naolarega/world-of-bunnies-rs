use crate::{
    constants::{random_name, Color, Sex},
    utils::{simple_rng::SimpleRNG, RandomNumberGenerator},
};

pub struct Bunny {
    sex: Sex,
    color: Color,
    age: u8,
    name: &'static str,
    radioactive: bool,
}

impl Bunny {
    pub fn new() -> Self {
        let mut rng = SimpleRNG::from_current_time();

        Self {
            sex: Sex::random_sex(&mut rng),
            color: Color::random_color(&mut rng),
            age: 0,
            name: random_name(&mut rng),
            radioactive: false,
        }
    }
}
