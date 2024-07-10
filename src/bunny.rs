use crate::{
    constants::{random_name, Color, Sex, WorldError},
    utils::{simple_rng::SimpleRNG, RandomNumberGenerator},
};

pub struct Bunny {
    pub id: u32,
    pub sex: Sex,
    pub color: Color,
    pub age: u8,
    pub name: &'static str,
    pub radioactive: bool,
}

impl Bunny {
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

impl From<&Bunny> for Bunny {
    fn from(mother_bunny: &Bunny) -> Self {
        let mut bunny_builder = BunnyBuilder::new();

        bunny_builder.color(mother_bunny.color);

        bunny_builder.build()
    }
}

pub struct BunnyBuilder {
    sex: Option<Sex>,
    color: Option<Color>,
    name: Option<&'static str>,
    radioactive: Option<bool>,
}

impl BunnyBuilder {
    pub fn new() -> Self {
        Self {
            sex: None,
            color: None,
            name: None,
            radioactive: None,
        }
    }

    pub fn build(self) -> Bunny {
        let mut rng = SimpleRNG::from_current_time();

        Bunny {
            id: rng.next_u32(),
            sex: self.sex.unwrap_or(Sex::random_sex(&mut rng)),
            color: self.color.unwrap_or(Color::random_color(&mut rng)),
            age: 0,
            name: self.name.unwrap_or(random_name(&mut rng)),
            radioactive: self
                .radioactive
                .unwrap_or(matches!(rng.gen_range(0, 100), 0 | 1)),
        }
    }

    pub fn sex(&mut self, sex: Sex) -> &mut Self {
        self.sex = Some(sex);

        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);

        self
    }

    pub fn name(&mut self, name: &'static str) -> &mut Self {
        self.name = Some(name);

        self
    }

    pub fn radioactive(&mut self, radioactive: bool) -> &mut Self {
        self.radioactive = Some(radioactive);

        self
    }
}

#[cfg(test)]
mod tests {
    use super::{Bunny, BunnyBuilder};

    #[test]
    fn bunny_generation() {
        let bunny_sample = 50;
        let mut bunnies = Vec::<Bunny>::with_capacity(bunny_sample);

        for _ in 0..bunny_sample {
            bunnies.push(BunnyBuilder::new().build());
        }

        for bunny in bunnies {
            println!(
                "Bunny {} [ sex: {:?}, color: {:?}, age: {}, radioactive: {} ]",
                bunny.name, bunny.sex, bunny.color, bunny.age, bunny.radioactive
            );
        }
    }
}
