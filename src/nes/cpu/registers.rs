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
pub struct Registers {
    A: u8,
    X: u8,
    Y: u8,
    Sp: u8,
    Pc: u16,
    P: Status,
}

#[derive(Debug)]
pub enum ByteRegister {
    A,
    X,
    Y,
    SP,
    P,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            A: 0,
            X: 0,
            Y: 0,
            Pc: 0x8000,
            Sp: 0xFD,
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

    pub fn reset(&mut self) {
        self.A = 0;
        self.X = 0;
        self.Y = 0;
        self.Pc = 0x8000;
        self.Sp = 0xFD;
        self.P.negative = false;
        self.P.overflow = false;
        self.P.reserved = true;
        self.P.break_mode = true;
        self.P.decimal_mode = false;
        self.P.interrupt = true;
        self.P.zero = false;
        self.P.carry = false;
    }

    pub fn get(&self, name: ByteRegister) -> u8 {
        let bool_to_u8 = |v: bool| if v { 1 } else { 0 };
        match name {
            ByteRegister::A => self.A,
            ByteRegister::X => self.X,
            ByteRegister::Y => self.Y,
            ByteRegister::SP => self.Sp,
            ByteRegister::P => {
                bool_to_u8(self.P.negative) << 7 | bool_to_u8(self.P.overflow) << 6 |
                bool_to_u8(self.P.reserved) << 5 |
                bool_to_u8(self.P.break_mode) << 4 |
                bool_to_u8(self.P.decimal_mode) << 3 |
                bool_to_u8(self.P.interrupt) << 2 | bool_to_u8(self.P.zero) << 1 |
                bool_to_u8(self.P.carry) as u8
            }
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.Pc
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

    pub fn set_pc(&mut self, v: u16) -> &mut Self {
        self.Pc = v;
        self
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
        self.Sp = v;
        self
    }

    pub fn set_negative(&mut self) -> &mut Self {
        self.P.negative = true;
        self
    }

    pub fn set_overflow(&mut self) -> &mut Self {
        self.P.overflow = true;
        self
    }

    pub fn set_reserved(&mut self) -> &mut Self {
        self.P.reserved = true;
        self
    }

    pub fn set_break(&mut self) -> &mut Self {
        self.P.break_mode = true;
        self
    }

    pub fn set_interrupt(&mut self) -> &mut Self {
        self.P.interrupt = true;
        self
    }

    pub fn set_zero(&mut self) -> &mut Self {
        self.P.zero = true;
        self
    }

    pub fn set_carry(&mut self) -> &mut Self {
        self.P.carry = true;
        self
    }

    pub fn update_negative(&mut self, v: u8) -> &mut Self {
        self.P.negative = v & 0x80 == 0x80;
        self
    }

    pub fn update_zero(&mut self, v: u8) -> &mut Self {
        self.P.zero = v == 0;
        self
    }

    pub fn update_pc(&mut self) -> &mut Self {
        self.Pc += 1;
        self
    }

    pub fn inc_sp(&mut self) -> &mut Self {
        self.Sp += 1;
        self
    }

    pub fn dec_sp(&mut self) -> &mut Self {
        self.Sp -= 1;
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