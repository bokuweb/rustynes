
pub struct Nes {
    pub a: usize,
}

impl Nes {
    pub fn new() -> Nes {
        Nes { a: 10 }
    }

    pub fn add(&mut self) -> usize {
        self.a += 1;
        self.a
    }
}
