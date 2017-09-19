mod opecode;

use std::collections::HashMap;
use nes::bus::cpu_bus::CpuBus;
// use self::opecode;


// pub enum ReadMode {
//     Byte,
//     Word,
// }

struct Status {
    negative: bool,
    overflow: bool,
    reserved: bool,
    break_mode: bool,
    decimal_mode: bool,
    interrupt: bool,
    zero: bool,
    carry: bool,
}

struct Registers {
    A: u8,
    X: u8,
    Y: u8,
    PC: u16,
    SP: u16,
    P: Status,
}

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { registers: Cpu::create_default_registers() }
    }

    pub fn reset(&mut self, bus: &CpuBus) {
        self.registers = Cpu::create_default_registers();
        self.registers.PC = self.read_word(bus, 0xFFFC);
    }

    pub fn run(&self, mut bus: &CpuBus) -> usize {
        let ref m = opecode::opecode::MAP;
        let a = 0xA5;
        println!("{:?}", *m.get(&a).unwrap());
        20
    }

    fn read_byte(&self, bus: &CpuBus, addr: u16) -> u8 {
        bus.read(addr)
    }

    fn read_word(&self, bus: &CpuBus, addr: u16) -> u16 {
        let low = bus.read(addr) as u16;
        let high = bus.read(addr + 1) as u16;
        (high << 8 | low) as u16
    }

    fn create_default_registers() -> Registers {
        Registers {
            A: 0,
            X: 0,
            Y: 0,
            PC: 0x8000,
            SP: 0x01FD,
            P: Status {
                negative: false,
                overflow: false,
                reserved: true,
                break_mode: true,
                decimal_mode: false,
                interrupt: true,
                zero: false,
                carry: false,
            },
        }
    }
}
