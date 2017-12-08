mod parser;
mod rom;
mod ram;
mod bus;
mod cpu;
mod ppu;
mod cpu_registers;
mod keypad;
mod renderer;
mod types;
mod helper;

pub use self::ppu::background;
pub use self::ppu::Tile;
pub use self::ppu::{SpriteWithCtx, Sprite, SpritePosition};
pub use self::keypad::*;

use self::ppu::*;
use self::renderer::*;
use self::rom::Rom;
use self::ram::Ram;
use self::bus::cpu_bus;
use nes::types::Data;

#[derive(Debug)]
pub struct Context {
    ppu: Ppu,
    program_rom: Box<Rom>,
    work_ram: Box<Ram>,
    cpu_registers: cpu_registers::Registers,
    keypad: Keypad,
}

pub fn reset(ctx: &mut Context) {
    let mut cpu_bus = cpu_bus::Bus::new(&ctx.program_rom,
                                        &ctx.work_ram,
                                        &mut ctx.ppu,
                                        &mut ctx.keypad);
    cpu::reset(&mut ctx.cpu_registers, &mut cpu_bus);
}

pub fn run(ctx: &mut Context, key_state: u8) {
    let mut cycle = 0;
    loop {
        {
            ctx.keypad.update(key_state);
        }
        {
            let mut cpu_bus = cpu_bus::Bus::new(&ctx.program_rom,
                                                &ctx.work_ram,
                                                &mut ctx.ppu,
                                                &mut ctx.keypad);
            cycle += cpu::run(&mut ctx.cpu_registers, &mut cpu_bus);
        }
        let is_ready = ctx.ppu.run((cycle * 3) as usize);
        if is_ready {
            render(&ctx.ppu.background.0, &ctx.ppu.sprites);
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
            ppu: Ppu::new(cassette.character_ram,
                          PpuConfig { is_horizontal_mirror: cassette.is_horizontal_mirror }),
            work_ram: Box::new(Ram::new(vec![0; 0x0800])),
            keypad: Keypad::new(),
        }
    }
}
