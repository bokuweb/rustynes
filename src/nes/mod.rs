#![feature(box_syntax)]

mod parser;
mod rom;
mod ram;
mod bus;
mod cpu;
mod ppu;

use self::cpu::Cpu;
use self::ppu::Ppu;
use self::rom::Rom;
use self::ram::Ram;
use self::bus::cpu_bus::CpuBus;

pub struct Nes {
    cpu: Cpu,
    ppu: Ppu,
    program_rom: Rom,
    work_ram: Ram,
    character_memory: Ram,
}

impl Nes {
    pub fn new(buf: &mut [u8]) -> Nes {
        let cassette = parser::parse(buf);
        Nes {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            program_rom: Rom::new(cassette.program_rom),
            work_ram: Ram::new(vec![0; 0x0800]),
            character_memory: Ram::new(cassette.character_memory),
        }
    }

    pub fn run(&mut self) {
        let mut cycle = 0;
        let mut cpu_bus = CpuBus::new(
            &self.program_rom,
            &mut self.character_memory,
            &mut self.work_ram,
            &mut self.ppu,
        );
        loop {
            cycle += self.cpu.run(&mut cpu_bus);
            if cycle > 300 {
                println!("{}", cycle);
                break;
            }
        }
    }
}
