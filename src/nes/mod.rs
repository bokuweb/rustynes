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
use std::rc::Rc;
use std::cell::RefCell;
use nes::types::{Data, Addr, Word};
use nes::helper::*;

#[derive(Debug)]
pub struct Nes {
    cpu: RefCell<Cpu>,
    ppu: RefCell<Ppu>,
    work_ram: RefCell<Ram>,
    character_ram: RefCell<Ram>,
    program_rom: Rom,
}

impl Nes {
    pub fn new(buf: &mut [Data]) -> Nes {
        let cassette = parser::parse(buf);
        // let character_ram = parser::parse(buf).character_ram;
        Nes {
            cpu: RefCell::new(Cpu::new()),
            ppu: RefCell::new(Ppu::new()),
            program_rom: Rom::new(cassette.program_rom),
            work_ram: RefCell::new(Ram::new(vec![0; 0x0800])),
            character_ram: RefCell::new(Ram::new(vec![0; 0x0800])),
        }
    }

    pub fn reset(&self) {
        // TODO: let mut cpu_bus = self.create_bus();
        // let cpu_bus = CpuBus::new(&self.program_rom,
        //                           &self.character_ram,
        //                           &self.work_ram,
        //                           &self.ppu);
        self.cpu.borrow_mut().reset(|addr: Addr| self.read(addr));
    }

    pub fn run(&self) {
        let mut cycle = 0;
        // let cpu_bus = CpuBus::new(&self.program_rom,
        //                           &self.character_ram,
        //                           &self.work_ram,
        //                           &self.ppu);
        loop {
            cycle += self.cpu
                .borrow_mut()
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
