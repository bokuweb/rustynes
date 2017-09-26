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
        let pc = self.read_word(bus, 0xFFFC);
        println!("Start from {:?}", pc);
        self.registers.PC = self.read_word(bus, 0xFFFC);
    }

    pub fn run(&mut self, mut bus: &CpuBus) -> usize {
        let code = self.fetch(bus);
        let ref map = opecode::opecode::MAP;
        println!("hoge");
        println!("{:?}", code);
        println!("{:?}", *map.get(&code).unwrap());
        20
    }

    fn fetch(&mut self, bus: &CpuBus) -> u8 {
        println!("{}", self.registers.PC);
        let code = self.read_byte(bus, self.registers.PC);
        self.registers.PC += 1;
        code
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
