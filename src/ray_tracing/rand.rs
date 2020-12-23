use rand::*;

pub struct Random(rngs::ThreadRng);

impl Default for Random {
    fn default() -> Random {
        Random(rngs::ThreadRng::default())
    }
}

impl Random {
    pub fn random_double(&mut self) -> f32 {
        self.0.gen()
    }

    pub fn random_int_in(&mut self, min: usize, max: usize) -> usize {
        self.random_double_in(min as f32, max as f32 + 1.0) as usize
    }
    pub fn random_double_in(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.random_double()
    }
}
