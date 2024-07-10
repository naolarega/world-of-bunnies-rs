use crate::{
    bunny::{Bunny, BunnyBuilder},
    constants::{Sex, WorldError},
    utils::{simple_rng::SimpleRNG, RandomNumberGenerator},
};

pub enum Events {
    Born {
        name: &'static str,
        radioactive: bool,
    },
    Died {
        name: &'static str,
        radioactive: bool,
    },
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
                    bunnies.push(BunnyBuilder::new().build());
                }

                bunnies
            },
        }
    }

    pub fn progress(&mut self) -> Result<Vec<Events>, WorldError> {
        let mut bunny_events = self.age_bunnies();

        if self.bunnies.is_empty() {
            return Err(WorldError::AllBunniesDead);
        }

        if self.bunnies_can_reproduce() {
            bunny_events.extend(self.reproduce());
        }

        if let Some((radioactive_vampire_bunnies, rest_of_the_bunnies)) =
            self.radioactive_vampire_bunnies_exist()
        {
            if rest_of_the_bunnies > 0 {
                self.contaminte_bunny(radioactive_vampire_bunnies, rest_of_the_bunnies);
            }
        }

        if self.bunnies.len() > 1000 {
            bunny_events.extend(self.cull_half_population());
        }

        Ok(bunny_events)
    }

    pub fn report(&self) {
        println!("## Bunnies world report ##\n");

        self.bunnies.iter().for_each(|bunny| {
            println!(
                "{}, {}, {:?}, {:?}, {}",
                bunny.name,
                bunny.age,
                bunny.sex,
                bunny.color,
                if bunny.radioactive {
                    "is radioactive vampire"
                } else {
                    "regular"
                }
            )
        });

        println!("\n** Total bunnies: {} **\n", self.bunnies.len());
        println!("## End of bunnies world report ##\n");
    }

    fn cull_half_population(&mut self) -> Vec<Events> {
        let mut bunnies_to_cull = self.bunnies.len() / 2;
        let mut rng = SimpleRNG::from_current_time();
        let mut bunny_events = Vec::with_capacity(bunnies_to_cull);

        loop {
            if bunnies_to_cull == 0 {
                break;
            }

            let random_bunny_index = rng.gen_range(0, self.bunnies.len() as u32) as usize;
            let killed_bunny = self.bunnies.remove(random_bunny_index);

            bunny_events.push(Events::Died {
                name: killed_bunny.name,
                radioactive: killed_bunny.radioactive,
            });

            bunnies_to_cull -= 1;
        }

        println!("** Half the bunny population is culled **");

        bunny_events
    }

    fn contaminte_bunny(&mut self, bunnies_to_contaminate: usize, regular_bunnies: usize) {
        let mut rng = SimpleRNG::from_current_time();
        let total_bunnies = self.bunnies.len() as u32;
        let mut regular_bunnies_left = regular_bunnies;

        for _ in 0..bunnies_to_contaminate {
            if regular_bunnies_left == 0 {
                return;
            }

            loop {
                if let Some(bunny) = self
                    .bunnies
                    .get_mut(rng.gen_range(0, total_bunnies) as usize)
                {
                    if bunny.radioactive {
                        continue;
                    }

                    bunny.radioactive = true;
                    regular_bunnies_left -= 1;

                    break;
                }
            }
        }
    }

    fn radioactive_vampire_bunnies_exist(&self) -> Option<(usize, usize)> {
        let radioactive_vampire_bunnies = self
            .bunnies
            .iter()
            .filter(|bunny| bunny.radioactive)
            .count();

        Some((
            radioactive_vampire_bunnies,
            self.bunnies.len() - radioactive_vampire_bunnies,
        ))
    }

    fn reproduce(&mut self) -> Vec<Events> {
        let new_born_bunnies = self
            .bunnies
            .iter()
            .filter_map(|bunny| {
                if bunny.age < 2 || bunny.radioactive || matches!(bunny.sex, Sex::Male) {
                    None
                } else {
                    Some(Bunny::from(bunny))
                }
            })
            .collect::<Vec<Bunny>>();
        let bunny_event = new_born_bunnies
            .iter()
            .map(|bunny| Events::Born {
                name: bunny.name,
                radioactive: bunny.radioactive,
            })
            .collect::<Vec<Events>>();

        self.bunnies.extend(new_born_bunnies);

        bunny_event
    }

    fn bunnies_can_reproduce(&self) -> bool {
        let mut male_bunnies = 0;
        let mut female_bunnies = 0;

        for bunny in self.bunnies.iter() {
            if bunny.age < 2 || bunny.radioactive {
                continue;
            }

            match bunny.sex {
                Sex::Male => male_bunnies += 1,
                Sex::Female => female_bunnies += 1,
            }

            if male_bunnies != 0 && female_bunnies != 0 {
                return true;
            }
        }

        false
    }

    fn age_bunnies(&mut self) -> Vec<Events> {
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
                name: pruned_bunny.name,
                radioactive: pruned_bunny.radioactive,
            });
        }

        pruned_bunnies
    }
}
