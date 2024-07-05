use crate::{
    bunny::Bunny,
    constants::{Sex, WorldError},
};

pub enum Events {
    Born { id: u32, name: &'static str },
    Died { id: u32, name: &'static str },
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
        let mut bunny_events = self.age_bunnies();

        if self.bunnies_can_reproduce() {}

        Ok(bunny_events)
    }

    fn bunnies_can_reproduce(&self) -> bool {
        let mut male_bunnies = 0;
        let mut female_bunnies = 0;

        for bunnie in self.bunnies.iter() {
            if bunnie.age < 2 {
                continue;
            }

            match bunnie.sex {
                Sex::Male => male_bunnies += 1,
                Sex::Female => female_bunnies += 1,
            }

            if male_bunnies != 0 && female_bunnies != 0 {
                return true;
            }
        }

        false
    }

    pub fn age_bunnies(&mut self) -> Vec<Events> {
        let mut dead_bunny_ids = Vec::<u32>::new();

        for bunny in self.bunnies.iter_mut() {
            if let Err(WorldError::BunnyDead) = bunny.age_by_a_year() {
                dead_bunny_ids.push(bunny.id);
            }
        }

        self.prune_dead_bunnies(dead_bunny_ids)
    }

    fn prune_dead_bunnies(&mut self, dead_bunny_ids: Vec<u32>) -> Vec<Events> {
        let mut pruned_bunnies = Vec::with_capacity(dead_bunny_ids.len());

        for id in dead_bunny_ids {
            let idx = if let Some((idx, _)) = self
                .bunnies
                .iter()
                .enumerate()
                .find(|(_, bunny)| bunny.id == id)
            {
                idx
            } else {
                continue;
            };

            let pruned_bunny = self.bunnies.remove(idx);

            pruned_bunnies.push(Events::Died {
                id: pruned_bunny.id,
                name: pruned_bunny.name,
            });
        }

        pruned_bunnies
    }
}
