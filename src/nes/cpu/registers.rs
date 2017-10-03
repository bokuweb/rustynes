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
    Pc: u16,
    Sp: u16,
    P: Status,
}

#[derive(Debug)]
pub enum ByteRegister {
    A,
    X,
    Y,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            A: 0,
            X: 0,
            Y: 0,
            Pc: 0x8000,
            Sp: 0x01FD,
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
        self.Sp = 0x01FD;
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
        match name {
            ByteRegister::A => self.A,
            ByteRegister::X => self.X,
            ByteRegister::Y => self.Y,
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.Pc
    }

    pub fn get_sp(&self) -> u16 {
        self.Sp
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

    pub fn set_sp(&mut self, v: u16) -> &mut Self {
        self.Sp = v;
        self
    }

    pub fn set_break(&mut self) -> &mut Self {
        self.P.break_mode = true;
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
}
