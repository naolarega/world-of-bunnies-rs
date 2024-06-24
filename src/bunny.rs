use crate::{
    constants::{random_name, Color, Sex, WorldError},
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
            radioactive: match rng.gen_range(0, 100) {
                0 | 1 => true,
                _ => false,
            },
        }
    }

    pub fn age_by_a_year(&mut self) -> Result<u8, WorldError> {
        if !self.is_alive() {
            return Err(WorldError::BunnyDead);
        } else {
            self.age += 1;
        }

        Ok(self.age)
    }

    pub fn is_alive(&self) -> bool {
        self.age <= 10 || (self.radioactive && self.age <= 50)
    }
}

#[cfg(test)]
mod tests {
    use super::Bunny;

    #[test]
    fn bunny_generation() {
        let bunny_sample = 50;
        let mut bunnies = Vec::<Bunny>::with_capacity(bunny_sample);

        for _ in 0..bunny_sample {
            bunnies.push(Bunny::new());
        }

        for bunny in bunnies {
            println!(
                "Bunny {} [ sex: {:?}, color: {:?}, age: {}, radioactive: {} ]",
                bunny.name, bunny.sex, bunny.color, bunny.age, bunny.radioactive
            );
        }
    }
}
