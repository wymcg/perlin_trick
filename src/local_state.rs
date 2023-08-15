use rand::{Rng, thread_rng};

pub struct LocalState {
    pub red_seed: u32,
    pub green_seed: u32,
    pub blue_seed: u32,
    pub z_value: f64
}

impl LocalState {
    pub fn new_rand_seeds() -> Self {
        let mut rng = thread_rng();

        Self {
            red_seed: rng.gen(),
            green_seed: rng.gen(),
            blue_seed: rng.gen(),
            ..Default::default()
        }
    }
}

impl Default for LocalState {
    fn default() -> Self {
        Self {
            red_seed: 0,
            green_seed: 0,
            blue_seed: 0,
            z_value: 0.0
        }
    }
}