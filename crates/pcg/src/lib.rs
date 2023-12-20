use std::time::{SystemTime, UNIX_EPOCH};

// see https://en.wikipedia.org/wiki/Linear_congruential_generator
// using musl / newlib const
const PCG_MUL: u64 = 6364136223846793005;
const PCG_ADD: u64 = 1;

pub struct Pcg {
    rand_seed: u64,
}

impl Default for Pcg {
    fn default() -> Self {
        Self {
            rand_seed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u64,
        }
    }
}

impl Pcg {
    #[allow(dead_code)]
    pub fn genu64(&mut self) -> u64 {
        self.rand_seed = self
            .rand_seed
            .overflowing_mul(PCG_MUL)
            .0
            .overflowing_add(PCG_ADD)
            .0;
        self.rand_seed
    }

    #[allow(dead_code)]
    pub fn genu32(&mut self) -> u32 {
        self.genu64() as u32
    }

    #[allow(dead_code)]
    pub fn genf32(&mut self) -> f32 {
        self.genu64() as f32 / u64::MAX as f32
    }
}
