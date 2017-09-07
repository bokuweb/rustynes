mod parser;
mod rom;
mod ram;
mod bus;

use self::rom::Rom;
use self::ram::Ram;
use self::bus::cpu_bus::CpuBus;

pub struct Nes {
    character_memory: Ram,
    cpu_bus: CpuBus,
}

impl Nes {
    pub fn new(buf: &mut [u8]) -> Nes {
        let cassette = parser::parse(buf);
        let program_rom = Rom::new(cassette.program_rom);
        Nes {
            character_memory: Ram::new(cassette.character_memory),
            cpu_bus: CpuBus::new(program_rom),
        }
    }

    pub fn run(&self) -> u8 {
        self.cpu_bus.read(0)
    }
}
