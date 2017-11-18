// #![feature(box_syntax)]

mod parser;
mod rom;
mod ram;
mod bus;
mod cpu;
mod cpu_registers;
mod ppu;
// mod ppu_registers;
mod types;
mod helper;

use self::ppu::{Ppu, PpuConfig};
use self::rom::Rom;
use self::ram::Ram;
use self::bus::cpu_bus;
use nes::types::{Data, Addr};

// #[derive(Debug)]
// pub struct Nes {
//     context: Context,
// }

#[derive(Debug)]
pub struct Context {
    ppu: Ppu,
    program_rom: Box<Rom>,
    work_ram: Box<Ram>,
    cpu_registers: cpu_registers::Registers,
    // ppu_registers: ppu_registers::Registers,
}

#[derive(Debug)]
pub struct RenderingContext {
    ppu: Ppu,
    program_rom: Box<Rom>,
    work_ram: Box<Ram>,
    cpu_registers: cpu_registers::Registers,
    // ppu_registers: ppu_registers::Registers,
}

pub fn reset(ctx: &mut Context) {
    let mut cpu_bus = cpu_bus::Bus::new(&ctx.program_rom, &ctx.work_ram, &mut ctx.ppu);
    cpu::reset(&mut ctx.cpu_registers, &mut cpu_bus);
}

pub fn run(ctx: &mut Context) {
    let mut cycle = 0;
    loop {
        {
            let mut cpu_bus = cpu_bus::Bus::new(&ctx.program_rom, &ctx.work_ram, &mut ctx.ppu);
            cycle += cpu::run(&mut ctx.cpu_registers, &mut cpu_bus);
        }
        let is_ready = ctx.ppu.run((cycle * 3) as usize);
        if is_ready {
            break;
        }
    }
}

impl Context {
    pub fn new(buf: &mut [Data]) -> Self {
        let cassette = parser::parse(buf);
        // println!("{:?}", cassette.program_rom);
        Context {
            cpu_registers: cpu_registers::Registers::new(),
            program_rom: Box::new(Rom::new(cassette.program_rom)),
            ppu: Ppu::new(cassette.character_ram,
                          PpuConfig { is_horizontal_mirror: cassette.is_horizontal_mirror }),
            // ppu_registers: ppu_registers::egisters::new(),
            work_ram: Box::new(Ram::new(vec![0; 0x0800])),
        }
    }
}
