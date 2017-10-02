#[derive(Debug)]
pub struct Status {
    pub negative: bool,
    pub overflow: bool,
    pub reserved: bool,
    pub break_mode: bool,
    pub decimal_mode: bool,
    pub interrupt: bool,
    pub zero: bool,
    pub carry: bool,
}

#[derive(Debug)]
pub struct Registers {
    pub A: u8,
    pub X: u8,
    pub Y: u8,
    pub PC: u16,
    pub SP: u16,
    pub P: Status,
}

#[derive(Debug)]
pub enum RegisterName {
    A,
    X,
    Y,
    PC,
    SP,
    P,
}

impl Registers {
    pub fn new() -> Self {
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

    pub fn reset(&mut self) {
        self.A = 0;
        self.X = 0;
        self.Y = 0;
        self.PC = 0x8000;
        self.SP = 0x01FD;
        self.P.negative = false;
        self.P.overflow = false;
        self.P.reserved = true;
        self.P.break_mode = true;
        self.P.decimal_mode = false;
        self.P.interrupt = true;
        self.P.zero = false;
        self.P.carry = false;
    }

    pub fn get(&self, name: RegisterName) -> u8 {
        match name {
            RegisterName::A => self.A,
            RegisterName::X => self.X,
            RegisterName::Y => self.Y,
            _ => 0,
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.PC
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
        self.PC = v;
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
        self.PC += 1;
        self
    }    
}
