use super::super::super::types::{Data, Addr, Word};
use super::super::super::Ram;

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

    pub fn read(&mut self, vram: &Ram, cram: &Ram, addr: Addr) -> Data {
        // TODO: Has buffer implemented to each memories?
        let buf = self.buf;
        if addr >= 0x2000 {
            let addr = self.calc_addr(addr);
            if addr >= 0x3F00 {
                return vram.read(addr);
            }
            self.buf = vram.read(addr);
        } else {
            self.buf = cram.read(addr);
        }
        buf
    }

    pub fn write(&mut self, vram: &Ram, cram: &Ram, addr: Addr, data: Data) {
        if addr >= 0x2000 {
            if addr >= 0x3f00 && addr < 0x4000 {
                // this.palette.write(this.vramAddr - 0x3f00, data);
            } else {
                let addr = self.calc_addr(addr);
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
