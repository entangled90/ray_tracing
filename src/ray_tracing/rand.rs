use rand::{rngs::ThreadRng, Rng};

pub struct Random(ThreadRng);

impl Random {
    pub fn new() -> Random {
        Random(rand::thread_rng())
    }
    pub fn random_double(&mut self) -> f64 {
        self.0.gen()
    }

    pub fn random_double_in(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.random_double()
    }
}
