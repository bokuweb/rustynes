mod parser;
mod rom;

pub struct Nes {
    // pub cassette: parser::Cassette,
    program_rom: rom::Rom,
}

impl Nes {
    pub fn new(buf: &mut [u8]) -> Nes {
        let cassette = parser::parse(buf);
        Nes {
            // cassette: cassette,
            program_rom: rom::Rom::new(cassette.program_rom),
        }
    }

    pub fn run(&mut self) -> u8 {
        self.program_rom.read(0)
    }
}
