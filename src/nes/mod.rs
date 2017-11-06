// #![feature(box_syntax)]

mod parser;
mod rom;
mod ram;
mod bus;
mod cpu;
mod cpu_registers;
mod ppu;
mod types;
mod helper;

use self::ppu::Ppu;
use self::rom::Rom;
use self::ram::Ram;
use self::bus::cpu_bus;
use nes::types::{Data, Addr};

#[derive(Debug)]
pub struct Nes {
    context: Context,
}

#[derive(Debug)]
pub struct Context {
    ppu: Box<Ppu>,
    program_rom: Box<Rom>,
    work_ram: Box<Ram>,
    cpu_registers: cpu_registers::Registers,
}

pub fn reset(ctx: &mut Context) {
    let cpu_bus = cpu_bus::Bus::new(&ctx.program_rom,
                                  &ctx.work_ram,
                                  &ctx.ppu
                                  );
    cpu::reset(&mut ctx.cpu_registers, &cpu_bus);
}

pub fn run(ctx: &mut Context) {
    let mut cycle = 0;
    let mut cpu_bus = cpu_bus::Bus::new(&ctx.program_rom,
                                  &ctx.work_ram,
                                  &ctx.ppu
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
            cpu_registers: cpu_registers::Registers::new(),
            program_rom: Box::new(Rom::new(cassette.program_rom)),
            ppu: Box::new(Ppu::new(cassette.character_ram)),
            work_ram: Box::new(Ram::new(vec![0; 0x0800])),
        }
    }
}
