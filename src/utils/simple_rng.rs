use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils::RandomNumberGenerator;

pub struct SimpleRNG {
    state: u64,
}

impl SimpleRNG {
    pub fn new(seed: u64) -> Self {
        SimpleRNG { state: seed }
    }

    pub fn from_current_time() -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let seed = since_the_epoch.as_secs() + since_the_epoch.subsec_nanos() as u64;

        Self::new(seed)
    }
}

impl RandomNumberGenerator for SimpleRNG {
    fn next_u32(&mut self) -> u32 {
        // Constants for the LCG algorithm
        const A: u64 = 6364136223846793005;
        const C: u64 = 1;
        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        (self.state >> 32) as u32
    }

    fn gen_range(&mut self, min: u32, max: u32) -> u32 {
        let range = max - min;
        min + (self.next_u32() % range)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{RandomNumberGenerator, SimpleRNG};

    const NUMBERR_OF_SAMPLES: i32 = 100000;

    #[test]
    fn next_u32() {
        let mut rng = SimpleRNG::from_current_time();
        let mut frequencies: HashMap<u32, u32> = HashMap::new();

        for _ in 0..NUMBERR_OF_SAMPLES {
            *frequencies.entry(rng.next_u32()).or_insert(0) += 1;
        }

        for (number, count) in frequencies.iter() {
            if *count > 1 {
                println!("Number {}: appeared {} times", number, count);
            }
        }
    }

    #[test]
    fn gen_range() {
        let mut rng = SimpleRNG::from_current_time();
        let mut frequencies: HashMap<u32, u32> = HashMap::new();
        let min = 10;
        let max = 20;

        for _ in 0..NUMBERR_OF_SAMPLES {
            let number = rng.gen_range(min, max);
            *frequencies.entry(number).or_insert(0) += 1;
        }

        for number in min..max {
            let count = frequencies.get(&number).unwrap_or(&0);
            println!("Number {}: {}", number, count);
        }
    }
}
