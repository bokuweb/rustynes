use super::super::super::types::{Data, Addr};
use super::super::super::Ram;
use super::super::palette::*;

#[derive(Debug)]
pub struct PpuData {
    buf: Data,
}

// Data ($2007) <> read/write
// Common name: PPUDATA
// Description: PPU data port
// Access: read, write
// VRAM read/write data register. After access, the video memory address will increment by an amount determined by $2000:2.
impl PpuData {
    pub fn new() -> Self {
        PpuData { buf: 0 }
    }

    pub fn read<P: PaletteRam>(&mut self, vram: &Ram, cram: &Ram, addr: Addr, palette: &P) -> Data {
        let buf = self.buf;
        // println!("vram cpu read0 {:X}", addr);
        if addr >= 0x2000 {
            let addr = self.calc_addr(addr);
            // Reading palette data from $3F00-$3FFF works differently.
            // The palette data is placed immediately on the data bus, and hence no dummy read is required.
            // Reading the palettes still updates the internal buffer though, but the data placed in it is the mirrored nametable data
            // that would appear "underneath" the palette. (Checking the PPU memory map should make this clearer.)
            if addr >= 0x3F00 {
                // println!("vram cpu read {:X}", addr);
                self.buf = vram.read(addr);
                return palette.read(addr - 0x3f00);
            }
            self.buf = vram.read(addr);
        } else {
            self.buf = cram.read(addr);
        }
        buf
    }

    pub fn write<P: PaletteRam>(&mut self,
                                vram: &Ram,
                                cram: &Ram,
                                addr: Addr,
                                data: Data,
                                palette: &mut P) {
        if addr >= 0x2000 {
            if addr >= 0x3f00 && addr < 0x4000 {
                palette.write(addr - 0x3f00, data);
            } else {
                let addr = self.calc_addr(addr);
                // println!("vram write {:X}", addr);
                vram.write(addr, data);
            }
        } else {
            cram.write(addr, data);
        }
    }

    fn calc_addr(&self, addr: Addr) -> Addr {
        if addr >= 0x3000 && addr < 0x3f00 {
            addr - 0x3000
        } else {
            addr - 0x2000
        }
    }
}
