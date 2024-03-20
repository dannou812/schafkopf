use rand::prelude::ThreadRng;
use rand::Rng;

// used for encapsulation of internal random call, for the case of it needing to change
// wrapper for thread_rng for reusing it and thereby increasing performance
pub struct RNG {
    generator: ThreadRng,
}

impl RNG {
    pub fn new() -> Self {
        RNG {
            generator: rand::thread_rng(),
        }
    }

    pub fn unsigned_between(&mut self, lower_bound: u64, upper_bound: u64) -> u64 {
        self.generator.gen_range(lower_bound..upper_bound)
    }

    pub fn usize_between(&mut self, lower_bound: usize, upper_bound: usize) -> usize {
        self.generator.gen_range(lower_bound..upper_bound)
    }
}
