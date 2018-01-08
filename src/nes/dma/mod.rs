use super::types::{Data, Addr, Word};
use nes::ram::Ram;
use nes::ppu::Ppu;

#[derive(Debug)]
pub struct Dma {
    register: Data,
    should_run: bool,
}

impl Dma {
    pub fn new() -> Self {
        Dma {
            register: 0,
            should_run: false,
        }
    }

    pub fn write(&mut self, data: Data) {
        self.register = data;
        self.should_run = true;
    }

    pub fn should_run(&self) -> bool {
        self.should_run
    }    

    pub fn run(&mut self, ram: &Ram, ppu: &mut Ppu) {
        let addr = (self.register as u16) << 8;
        for i in 0..0x100 {
            ppu.transfer_sprite(i, ram.read(addr + i));
        }
        self.should_run = false;
    }
}
