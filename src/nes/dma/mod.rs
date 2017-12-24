use super::types::{Data, Addr, Word};
use nes::ram::Ram;
use nes::ppu::Ppu;

pub fn transfer(addr: Addr, ram: &Ram, ppu: &mut Ppu) {
    for i in 0..0x100 {
        ppu.transfer_sprite(i, ram.read(addr + i));
    }
}
