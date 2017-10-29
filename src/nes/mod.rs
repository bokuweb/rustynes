// #![feature(box_syntax)]

mod parser;
mod rom;
mod ram;
mod bus;
mod cpu;
mod cpu_registers;
// mod ppu;
mod types;
mod helper;

use self::cpu_registers::CpuRegisters;
// use self::ppu::Ppu;
use self::rom::Rom;
use self::ram::Ram;
use self::bus::cpu_bus::CpuBus;
// use std::rc::Rc;
// use std::cell::RefCell;
use nes::types::{Data, Addr};
// use nes::helper::*;

#[derive(Debug)]
pub struct Nes {
    context: Context,
}

#[derive(Debug)]
pub struct Context {
    // ppu: Ppu,
    program_rom: Rom,
    work_ram: Ram,
    cpu_registers: CpuRegisters,
}

fn read(ref mut program_rom: &mut Rom, ref mut work_ram: &mut Ram, addr: Addr) -> Data {
    let cpu_bus = CpuBus::new(&program_rom,
                                  &work_ram,
                                  //&ctx.ppu
                                  );
    cpu_bus.read(addr)
}

pub fn reset(mut ctx: &mut Context) {
    let cpu_bus = CpuBus::new(&ctx.program_rom,
                                  &ctx.work_ram,
                                  //&ctx.ppu
                                  );
    cpu::reset(&mut ctx.cpu_registers, &cpu_bus);
}

pub fn run(ref mut ctx: &mut Context) {
    let mut cycle = 0;
    let mut cpu_bus = CpuBus::new(&ctx.program_rom,
                                  &ctx.work_ram,
                                  //&ctx.ppu
                                  );
    loop {
        cycle += cpu::run(&mut ctx.cpu_registers, &mut cpu_bus);
        if cycle > 20 {
            break;
        }
    }
}


impl Context {
    pub fn new(buf: &mut [Data]) -> Self {
        let cassette = parser::parse(buf);
        Context {
            cpu_registers: CpuRegisters::new(),
            program_rom: Rom::new(cassette.program_rom),
            // ppu: Ppu::new(cassette.character_ram),
            work_ram: Ram::new(vec![0; 0x0800]),
        }
    }
}

// pub fn reset(&mut self) {
//     cpu::reset(&mut self.context.cpu_registers, |addr: Addr| read(&mut self.context.program_rom, &mut self.context.work_ram, addr));
// }

// fn write(&mut self, addr: Addr, data: Data) {
//     let cpu_bus = CpuBus::new(&self.context.program_rom,
//                               // &self.context.character_ram,
//                               &self.context.work_ram,
//     );
//     cpu_bus.write(addr, data);
// }
