/**
 * Why This Specific Multiplier?
 * 6364136223846793005 is derived from Knuth’s LCG recommendations and is known to work well in 64-bit random number generators.
 * Used in PCG (Permuted Congruential Generator) and GLIBC’s random function.
 * It provides a full cycle length of 2^64, meaning the sequence won’t repeat until after 2^64 numbers are generated.
 */
const LCG_MAGIC_NUMBER: u64 = 6364136223846793005;
const LCG_INCREMENT: u64 = 3;

#[derive(Clone, Copy, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn from_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn new() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let seed = now.as_nanos() as u64;

        Self::from_seed(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state
            .wrapping_mul(LCG_MAGIC_NUMBER)
            .wrapping_add(LCG_INCREMENT);
        self.state
    }

    pub fn next_usize(&mut self) -> usize {
        self.next_u64() as usize
    }

    pub fn next_f64(&mut self) -> f64 {
        let bits = self.next_u64() >> 11;
        bits as f64 * (1.0 / (1u64 << 53) as f64)
    }

    pub fn next_u64_in_range(&mut self, min: u64, max: u64) -> u64 {
        if min > max {
            panic!("min must be less than or equal to max");
        }

        let range = max - min + 1;
        min + (self.next_u64() % range)
    }

    pub fn next_usize_in_range(&mut self, min: usize, max: usize) -> usize {
        if min > max {
            panic!("min must be less than or equal to max");
        }

        let range = max - min + 1;
        min + ((self.next_usize() as usize) % range)
    }

    pub fn next_f64_in_range(&mut self, min: f64, max: f64) -> f64 {
        min + self.next_f64() * (max - min)
    }
}
