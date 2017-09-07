mod parser;

pub struct Nes {
    pub cassette: parser::Cassette,
}

impl Nes {
    pub fn new(buf: &mut [u8]) -> Nes {
        let cassette = parser::parse(buf);
        Nes { cassette: cassette }
    }

    pub fn run(&mut self) -> u8 {
        self.cassette.character_memory[1]
    }
}
