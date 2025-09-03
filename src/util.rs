use rand::{Rng, rngs::StdRng};

pub trait RandExt {
    fn flip(&mut self) -> bool;
}

impl RandExt for StdRng {
    fn flip(&mut self) -> bool {
        self.random_bool(0.5)
    }
}
