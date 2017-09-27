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
use std::rc::Rc;
use std::cell::RefCell;

pub struct Nes {
    pub cpu: Rc<RefCell<Cpu>>,
    pub ppu: Ppu,
    pub program_rom: Rom,
    pub work_ram: Ram,
    pub character_ram: Ram,
}

impl Nes {
    pub fn new(buf: &mut [u8]) -> Nes {
        let cassette = parser::parse(buf);
        // let program_rom = Box::new(parser::parse(buf).program_rom);
        let character_ram = Box::new(parser::parse(buf).character_ram);
        Nes {
            cpu: Rc::new(RefCell::new(Cpu::new())),
            ppu: Ppu::new(),
            program_rom: Rom::new(Box::new(parser::parse(buf).program_rom)),
            work_ram: Ram::new(Box::new(vec![0; 0x0800])),
            character_ram: Ram::new(character_ram),
        }
    }

    pub fn reset(&mut self) {
        // TODO: let mut cpu_bus = self.create_bus();
        let mut cpu_bus = CpuBus::new(&self.program_rom,
                                      &mut self.character_ram,
                                      &mut self.work_ram,
                                      &mut self.ppu);
        self.cpu.borrow_mut().reset(&mut cpu_bus);
    }

    pub fn run(&mut self) {
        let mut cycle = 0;
        let mut cpu_bus = CpuBus::new(&self.program_rom,
                                      &mut self.character_ram,
                                      &mut self.work_ram,
                                      &mut self.ppu);
        loop {
            println!("aa");
            println!("a{}", cycle);
            cycle += self.cpu.borrow_mut().run(&mut cpu_bus);
            println!("{}", cycle);
            if cycle > 20 {
                println!("{}", cycle);
                break;
            }
        }
    }

    fn create_bus(&mut self) -> CpuBus {
        CpuBus::new(&self.program_rom,
                    &mut self.character_ram,
                    &mut self.work_ram,
                    &mut self.ppu)
    }
}
