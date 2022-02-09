use rand::prelude::*;

pub fn rnd() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn rnd_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
