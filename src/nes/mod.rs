// #![feature(box_syntax)]

mod parser;
mod rom;
mod ram;
mod bus;
mod cpu;
mod ppu;
mod types;
mod helper;

use self::cpu::Cpu;
use self::ppu::Ppu;
use self::rom::Rom;
use self::ram::Ram;
use self::bus::cpu_bus::CpuBus;
// use std::rc::Rc;
use std::cell::RefCell;
use nes::types::{Data, Addr, Word};
// use nes::helper::*;

#[derive(Debug)]
pub struct Nes {
    cpu: Cpu,
    ppu: Ppu,
    work_ram: Ram,
    character_ram: Ram,
    program_rom: Rom,
}

impl Nes {
    pub fn new(buf: &mut [Data]) -> Nes {
        let cassette = parser::parse(buf);
        // let character_ram = parser::parse(buf).character_ram;
        Nes {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            program_rom: Rom::new(cassette.program_rom),
            work_ram: Ram::new(vec![0; 0x0800]),
            character_ram: Ram::new(vec![0; 0x0800]),
        }
    }

    pub fn reset(&self) {
        self.cpu.reset(|addr: Addr| self.read(addr));
    }

    pub fn run(&self) {
        let mut cycle = 0;
        loop {
            cycle += self.cpu
                .run(|addr: Addr| self.read(addr),
                     |addr: Addr, data: Data| self.write(addr, data));
            if cycle > 20 {
                break;
            }
        }
    }
    fn read(&self, addr: Addr) -> Data {
        let cpu_bus = CpuBus::new(&self.program_rom,
                                  &self.character_ram,
                                  &self.work_ram,
                                  &self.ppu);
        cpu_bus.read(addr)
    }

    fn write(&self, addr: Addr, data: Data) {
        let cpu_bus = CpuBus::new(&self.program_rom,
                                  &self.character_ram,
                                  &self.work_ram,
                                  &self.ppu);
        cpu_bus.write(addr, data);
    }
}
