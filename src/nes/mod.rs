mod parser;
mod rom;
mod ram;
mod bus;
mod cpu;

use self::cpu::Cpu;
use self::rom::Rom;
use self::ram::Ram;
use self::bus::cpu_bus::CpuBus;

pub struct Nes {
    cpu: Cpu,
}

impl Nes {
    pub fn new(buf: &mut [u8]) -> Nes {
        let cassette = parser::parse(buf);
        let cpu_bus = CpuBus::new(
            Rom::new(cassette.program_rom),
            Ram::new(cassette.character_memory),
            Ram::new(vec![0; 0x0800]),
        );
        Nes { cpu: Cpu::new(cpu_bus) }
    }

    pub fn run(&self) {
        self.cpu.run()
    }
}
