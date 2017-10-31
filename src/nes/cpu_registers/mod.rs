use super::types::{Data, Addr, Word};
use super::helper::*;

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
#[allow(non_snake_case)]
pub struct CpuRegisters {
    A: u8,
    X: u8,
    Y: u8,
    SP: u8,
    PC: u16,
    P: Status,
}

pub trait Register {
    fn get_PC(&self) -> u16;

    fn get_A(&self) -> u8;

    fn get_X(&self) -> u8;

    fn get_Y(&self) -> u8;

    fn get_SP(&self) -> u8;

    fn get_P(&self) -> u8;

    fn set_A(&mut self, v: u8) -> &mut Self;

    fn set_X(&mut self, v: u8) -> &mut Self;

    fn set_Y(&mut self, v: u8) -> &mut Self;

    fn set_PC(&mut self, v: u16) -> &mut Self;

    fn set_P(&mut self, v: u8) -> &mut Self;

    fn set_SP(&mut self, v: u8) -> &mut Self;

    fn set_negative(&mut self, v: bool) -> &mut Self;

    fn set_overflow(&mut self, v: bool) -> &mut Self;

    fn set_reserved(&mut self, v: bool) -> &mut Self;

    fn set_break(&mut self, v: bool) -> &mut Self;

    fn set_interrupt(&mut self, v: bool) -> &mut Self;

    fn set_zero(&mut self, v: bool) -> &mut Self;

    fn set_decimal(&mut self, v: bool) -> &mut Self;

    fn set_carry(&mut self, v: bool) -> &mut Self;

    fn update_negative_by(&mut self, v: u8) -> &mut Self;

    fn update_overflow_by(&mut self, fetched: u8, computed: u8) -> &mut Self;

    fn update_zero_by(&mut self, v: u8) -> &mut Self;

    fn inc_SP(&mut self) -> &mut Self;

    fn dec_SP(&mut self) -> &mut Self;

    fn inc_PC(&mut self) -> &mut Self;

    fn dec_PC(&mut self) -> &mut Self;
}

// #[derive(Debug)]
//  enum ByteRegister {
//     A,
//     X,
//     Y,
//     SP,
//     P,
// }

// #[derive(Debug)]
//  enum StatusName {
//     negative,
//     overflow,
//     reserved,
//     break_mode,
//     decimal_mode,
//     interrupt,
//     zero,
//     carry,
// }
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
}
impl Register for CpuRegisters {
    #[allow(non_snake_case)]
    fn get_PC(&self) -> u16 {
        self.PC
    }

    #[allow(non_snake_case)]
    fn get_A(&self) -> u8 {
        self.A
    }

    #[allow(non_snake_case)]
    fn get_X(&self) -> u8 {
        self.X
    }

    #[allow(non_snake_case)]
    fn get_Y(&self) -> u8 {
        self.X
    }

    #[allow(non_snake_case)]
    fn get_SP(&self) -> u8 {
        self.SP
    }

    #[allow(non_snake_case)]
    fn get_P(&self) -> u8 {
        bool_to_u8(self.P.negative) << 7 | bool_to_u8(self.P.overflow) << 6 |
        bool_to_u8(self.P.reserved) << 5 | bool_to_u8(self.P.break_mode) << 4 |
        bool_to_u8(self.P.decimal_mode) << 3 | bool_to_u8(self.P.interrupt) << 2 |
        bool_to_u8(self.P.zero) << 1 | bool_to_u8(self.P.carry) as u8
    }

    fn set_A(&mut self, v: u8) -> &mut Self {
        self.A = v;
        self
    }

    fn set_X(&mut self, v: u8) -> &mut Self {
        self.X = v;
        self
    }

    fn set_Y(&mut self, v: u8) -> &mut Self {
        self.Y = v;
        self
    }

    fn set_PC(&mut self, v: u16) -> &mut Self {
        self.PC = v;
        self
    }

    fn set_P(&mut self, v: u8) -> &mut Self {
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

    fn set_SP(&mut self, v: u8) -> &mut Self {
        self.SP = v;
        self
    }

    fn set_negative(&mut self, v: bool) -> &mut Self {
        self.P.negative = v;
        self
    }

    fn set_overflow(&mut self, v: bool) -> &mut Self {
        self.P.overflow = v;
        self
    }

    fn set_reserved(&mut self, v: bool) -> &mut Self {
        self.P.reserved = v;
        self
    }

    fn set_break(&mut self, v: bool) -> &mut Self {
        self.P.break_mode = v;
        self
    }

    fn set_interrupt(&mut self, v: bool) -> &mut Self {
        self.P.interrupt = v;
        self
    }

    fn set_zero(&mut self, v: bool) -> &mut Self {
        self.P.zero = v;
        self
    }

    fn set_decimal(&mut self, v: bool) -> &mut Self {
        self.P.decimal_mode = v;
        self
    }

    fn set_carry(&mut self, v: bool) -> &mut Self {
        self.P.carry = v;
        self
    }

    fn update_negative_by(&mut self, v: u8) -> &mut Self {
        self.P.negative = v & 0x80 == 0x80;
        self
    }

    fn update_overflow_by(&mut self, fetched: u8, computed: u8) -> &mut Self {
        self.P.overflow = !(((self.A ^ fetched) & 0x80) != 0) &&
                          (((self.A ^ computed) & 0x80)) != 0;
        self
    }

    fn update_zero_by(&mut self, v: u8) -> &mut Self {
        self.P.zero = v == 0;
        self
    }

    fn inc_SP(&mut self) -> &mut Self {
        self.SP += 1;
        self
    }

    fn dec_SP(&mut self) -> &mut Self {
        self.SP -= 1;
        self
    }

    fn inc_PC(&mut self) -> &mut Self {
        self.PC += 1;
        self
    }

    fn dec_PC(&mut self) -> &mut Self {
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