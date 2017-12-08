use super::types::{Data, Addr, Word};

#[derive(Debug)]
pub struct Keypad {
    addr: Addr,
    reset: bool,
    register: u8,
    buffer: u8,
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            addr: 0,
            reset: false,
            register: 0,
            buffer: 0,
        }
    }

    pub fn update(&mut self, data: Data) {
        self.buffer = data;
    }

    pub fn write(&mut self, data: Data) {
        if data & 0x01 == 0x01 {
            self.reset = true;
        } else if self.reset && data & 0x00 == 0x00 {
            self.reset = false;
            self.addr = 0;
            self.register = self.buffer;
        }
    }

    pub fn read(&mut self) -> u8 {
        let v = (0x01 << self.addr) as u8;
        let ret = ((self.register & v) >> self.addr) as u8;
        self.addr += 1;
        ret
    }
}
