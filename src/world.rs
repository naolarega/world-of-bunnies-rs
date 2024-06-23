use crate::bunny::Bunny;

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
}
