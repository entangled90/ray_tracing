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

    pub fn random_double_in(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.random_double()
    }
}
