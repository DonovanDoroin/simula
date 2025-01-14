use oorandom::Rand32;

#[derive(Clone)]
pub struct Prng {
    rng: Rand32,
}

impl Prng {
    pub fn new(seed: u64) -> Self {
        Prng {
            rng: Rand32::new(seed),
        }
    }

    pub fn rand_float(&mut self) -> f32 {
        self.rng.rand_float()
    }
}

impl Default for Prng {
    fn default() -> Self {
        Prng {
            rng: Rand32::new(rand::random()),
        }
    }
}
