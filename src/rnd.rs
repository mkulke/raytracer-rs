use std::cell::RefCell;
use std::time::{SystemTime, UNIX_EPOCH};

struct PseudoRng {
    next: f64,
}

impl PseudoRng {
    fn new() -> Self {
        let next = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_micros() as f64;
        Self { next }
    }

    fn rnd(&mut self) -> f64 {
        self.next = (self.next.sin() * 10000.).fract() + 1. / 2.;
        self.next
    }
}

thread_local!(static RNG: RefCell<PseudoRng> = RefCell::new(PseudoRng::new()));

pub fn rnd() -> f64 {
    RNG.with(|rng_cell| {
        let mut rng = rng_cell.borrow_mut();
        rng.rnd()
    })
}

pub fn rnd_range(min: f64, max: f64) -> f64 {
    let m = max - min + 1.;
    min + (rnd() % m)
}
