mod opecode;

use std::collections::HashMap;
use nes::bus::cpu_bus::CpuBus;
// use self::opecode;

// pub enum ReadMode {
//     Byte,
//     Word,
// }

#[derive(Debug)]
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

#[derive(Debug)]
struct Registers {
    A: u8,
    X: u8,
    Y: u8,
    PC: u16,
    SP: u16,
    P: Status,
}

#[derive(Debug)]
pub struct Cpu {
    // registers: Box<Registers>,
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers {
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
            },
        }
    }

    pub fn reset(&mut self, bus: &CpuBus) {
        // self.reset_registers();
        let pc = self.read_word(bus, 0xFFFC);
        println!("Initial PC {}", pc);
        println!("registers {:?}", self.registers);
        self.registers.PC = 0x8000; //pc;
    }

    pub fn run(&mut self, mut bus: &CpuBus) -> u8 {
        println!("registers {:?}", self.registers);
        let code = self.fetch(bus);
        let ref map = opecode::opecode::MAP;
        let code = &*map.get(&code).unwrap();
        println!("{:?}", code);
        code.cycle
    }

    fn fetch(&mut self, bus: &CpuBus) -> u8 {
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


    fn reset_registers(&mut self) {
        self.registers.A = 0;
        self.registers.X = 0;
        self.registers.Y = 0;
        self.registers.PC = 0x8000;
        self.registers.SP = 0x01FD;
        self.registers.P.negative = false;
        self.registers.P.overflow = false;
        self.registers.P.reserved = true;
        self.registers.P.break_mode = true;
        self.registers.P.decimal_mode = false;
        self.registers.P.interrupt = true;
        self.registers.P.zero = false;
        self.registers.P.carry = false;
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
