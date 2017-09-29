// #![feature(box_syntax)]

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
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Nes {
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub program_rom: Rom,
    pub work_ram: Ram,
    pub character_ram: Ram,
}

impl Nes {
    pub fn new(/*buf: &mut [u8]*/) -> Nes {
        // let cassette = parser::parse(buf);
        // let character_ram = parser::parse(buf).character_ram;
        Nes {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            program_rom: Rom::new(vec![120; 0x8000]),
            work_ram: Ram::new(vec![0; 0x0800]),
            character_ram: Ram::new(vec![0; 0x0800]),
        }
    }

    pub fn reset(&mut self) {
        // TODO: let mut cpu_bus = self.create_bus();
        let mut cpu_bus = CpuBus::new(&self.program_rom,
                                      &mut self.character_ram,
                                      &mut self.work_ram,
                                      &mut self.ppu);
        self.cpu.reset(&mut cpu_bus);
    }

    pub fn run(&mut self) {
        let mut cycle = 0;
        let mut cpu_bus = CpuBus::new(&self.program_rom,
                                      &mut self.character_ram,
                                      &mut self.work_ram,
                                      &mut self.ppu);
        loop {
            cycle += self.cpu.run(&mut cpu_bus);
            if cycle > 20 {
                break;
            }
        }
    }

}
