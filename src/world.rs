use crate::{bunny::Bunny, constants::WorldError};

pub enum Events<'a> {
    Born(&'a Bunny),
    Died(&'a Bunny),
}

pub struct World {
    bunnies: Vec<Bunny>,
}

impl World {
    pub fn new() -> Self {
        Self {
            bunnies: {
                let mut bunnies = Vec::with_capacity(5);

                for _ in 0..5 {
                    bunnies.push(Bunny::new());
                }

                bunnies
            },
        }
    }

    pub fn progress(&mut self) -> Result<Vec<Events>, WorldError> {
        Ok(vec![])
    }

    pub fn age_bunnies(&mut self) {
        for bunny in self.bunnies.iter_mut() {
            bunny.age_by_a_year();
        }
    }
}
