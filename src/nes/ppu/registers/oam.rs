use super::super::super::types::{Data, Addr, Word};
use super::super::super::ram::*;

#[derive(Debug)]
pub struct Oam {
    addr: Addr,
}


impl Oam {
    pub fn new() -> Self {
        Oam { addr: 0 }
    }

    // OAM address ($2003) > write
    // Common name: OAMADDR
    // Description: OAM address port
    // Access: write
    // Write the address of OAM you want to access here. Most games just write $00 here and then use OAMDMA.
    // (DMA is implemented in the 2A03/7 chip and works by repeatedly writing to OAMDATA)
    // OAMADDR is set to 0 during each of ticks 257-320 (the sprite tile loading interval) of the pre-render and visible scanlines.
    pub fn reset_addr(&mut self) {
        self.addr = 0;
    }

    pub fn get_addr(&self) -> Addr {
        self.addr
    }

    pub fn write_addr(&mut self, data: Data) {
        self.addr += data as Addr;
    }

    // OAM data ($2004) <> read/write
    // Common name: OAMDATA
    // Description: OAM data port
    // Access: read, write
    // Write OAM data here. Writes will increment OAMADDR after the write;
    // reads during vertical or forced blanking return the value from OAM at that address but do not increment.
    pub fn write_data(&mut self, ram: &mut Ram, data: Data) {
        ram.write(self.addr, data);
        self.addr += 1;
    }

    pub fn read_data(&self, ram: &Ram) -> Data {
        ram.read(self.addr)
    }
}


#[test]
fn set_addr() {
    let mut reg = Oam::new();
    reg.write_addr(0xaa);
    assert_eq!(reg.get_addr(), 0xaa);
}
