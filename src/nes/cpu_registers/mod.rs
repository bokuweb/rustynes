use super::types::{Data, Addr, Word};
use super::helper::*;

#[derive(Debug)]
pub struct Status {
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
#[allow(non_snake_case)]
pub struct CpuRegisters {
    A: u8,
    X: u8,
    Y: u8,
    SP: u8,
    PC: u16,
    P: Status,
}

// #[derive(Debug)]
// pub enum ByteRegister {
//     A,
//     X,
//     Y,
//     SP,
//     P,
// }

#[derive(Debug)]
pub enum StatusName {
    negative,
    overflow,
    reserved,
    break_mode,
    decimal_mode,
    interrupt,
    zero,
    carry,
}

impl CpuRegisters {
    pub fn new() -> Self {
        CpuRegisters {
            A: 0,
            X: 0,
            Y: 0,
            PC: 0x8000,
            SP: 0xFD,
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

    pub fn get_status(&self, name: StatusName) -> bool {
        match name {
            StatusName::negative => self.P.negative,
            StatusName::overflow => self.P.overflow,
            StatusName::reserved => self.P.reserved,
            StatusName::break_mode => self.P.break_mode,
            StatusName::decimal_mode => self.P.decimal_mode,
            StatusName::interrupt => self.P.interrupt,
            StatusName::zero => self.P.zero,
            StatusName::carry => self.P.carry,
        }
    }

    pub fn get_PC(&self) -> u16 {
        self.PC
    }

    pub fn get_A(&self) -> u8 {
        self.A
    }

    pub fn get_X(&self) -> u8 {
        self.X
    }

    pub fn get_Y(&self) -> u8 {
        self.X
    }

    pub fn get_SP(&self) -> u8 {
        self.SP
    }

    pub fn get_P(&self) -> u8 {
        bool_to_u8(self.P.negative) << 7 | bool_to_u8(self.P.overflow) << 6 |
        bool_to_u8(self.P.reserved) << 5 | bool_to_u8(self.P.break_mode) << 4 |
        bool_to_u8(self.P.decimal_mode) << 3 | bool_to_u8(self.P.interrupt) << 2 |
        bool_to_u8(self.P.zero) << 1 | bool_to_u8(self.P.carry) as u8
    }

    pub fn set_acc(&mut self, v: u8) -> &mut Self {
        self.A = v;
        self
    }

    pub fn set_x(&mut self, v: u8) -> &mut Self {
        self.X = v;
        self
    }

    pub fn set_y(&mut self, v: u8) -> &mut Self {
        self.Y = v;
        self
    }

    pub fn set_pc(&mut self, v: u16) {
        self.PC = v;
    }

    pub fn set_p(&mut self, v: u8) -> &mut Self {
        self.P.negative = v & 0x80 == 0x80;
        self.P.overflow = v & 0x40 == 0x40;
        self.P.reserved = v & 0x20 == 0x20;
        self.P.break_mode = v & 0x10 == 0x10;
        self.P.decimal_mode = v & 0x08 == 0x08;
        self.P.interrupt = v & 0x04 == 0x04;
        self.P.zero = v & 0x02 == 0x02;
        self.P.carry = v & 0x01 == 0x01;
        self
    }

    pub fn set_sp(&mut self, v: u8) -> &mut Self {
        self.SP = v;
        self
    }

    pub fn set_negative(&mut self, v: bool) -> &mut Self {
        self.P.negative = v;
        self
    }

    pub fn set_overflow(&mut self, v: bool) -> &mut Self {
        self.P.overflow = v;
        self
    }

    pub fn set_reserved(&mut self, v: bool) -> &mut Self {
        self.P.reserved = v;
        self
    }

    pub fn set_break(&mut self, v: bool) -> &mut Self {
        self.P.break_mode = v;
        self
    }

    pub fn set_interrupt(&mut self, v: bool) -> &mut Self {
        self.P.interrupt = v;
        self
    }

    pub fn set_zero(&mut self, v: bool) -> &mut Self {
        self.P.zero = v;
        self
    }

    pub fn set_decimal(&mut self, v: bool) -> &mut Self {
        self.P.decimal_mode = v;
        self
    }

    pub fn set_carry(&mut self, v: bool) -> &mut Self {
        self.P.carry = v;
        self
    }

    pub fn update_negative(&mut self, v: u8) -> &mut Self {
        self.P.negative = v & 0x80 == 0x80;
        self
    }

    pub fn update_overflow(&mut self, fetched: u8, computed: u8) -> &mut Self {
        self.P.overflow = !(((self.A ^ fetched) & 0x80) != 0) &&
                          (((self.A ^ computed) & 0x80)) != 0;
        self
    }

    pub fn update_zero(&mut self, v: u8) -> &mut Self {
        self.P.zero = v == 0;
        self
    }

    pub fn update_PC(&mut self) -> &mut Self {
        self.PC += 1;
        self
    }

    pub fn inc_sp(&mut self) -> &mut Self {
        self.SP += 1;
        self
    }

    pub fn dec_sp(&mut self) -> &mut Self {
        self.SP -= 1;
        self
    }

    pub fn inc_pc(&mut self) -> &mut Self {
        self.PC += 1;
        self
    }

    pub fn dec_pc(&mut self) -> &mut Self {
        self.PC -= 1;
        self
    }
}

#[test]
fn get_p() {
    let mut reg = Registers::new();
    let p = reg.get(ByteRegister::P);
    assert_eq!(p, 0x34);
}

#[test]
fn update_zero() {
    let mut reg = Registers::new();
    reg.update_zero(0);
    let p = reg.get(ByteRegister::P);
    assert_eq!(p, 0x36);
}

#[test]
fn update_negative() {
    let mut reg = Registers::new();
    reg.update_negative(0x80);
    let p = reg.get(ByteRegister::P);
    assert_eq!(p, 0xB4);
}