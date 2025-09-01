use bracket_lib::random::RandomNumberGenerator;

pub trait RandExt {
    fn flip(&mut self) -> bool;
}

impl RandExt for RandomNumberGenerator {
    fn flip(&mut self) -> bool {
        self.range(0, 2) == 1
    }
}
