use super::super::types::{Addr, Data};

#[derive(Debug)]
pub struct Palette(Vec<u8>);

pub trait PaletteRam {
    fn read(&self, addr: Addr) -> Data;

    fn write(&mut self, addr: Addr, data: Data);
}

impl Palette {
    pub fn new() -> Self {
        Palette(Vec::new())
    }

    fn is_sprite_mirror(&self, addr: Addr) -> bool {
        (addr == 0x10) || (addr == 0x14) || (addr == 0x18) || (addr == 0x1c)
    }

    fn is_background_mirror(&self, addr: Addr) -> bool {
        (addr == 0x04) || (addr == 0x08) || (addr == 0x0c)
    }

    fn get_palette_addr(&self, addr: Addr) -> Addr {
        let mirror_downed = (addr & 0xFF) % 0x20;
        //NOTE: 0x3f10, 0x3f14, 0x3f18, 0x3f1c is mirror of 0x3f00, 0x3f04, 0x3f08, 0x3f0c
        if self.is_sprite_mirror(mirror_downed) {
            mirror_downed - 0x10
        } else {
            mirror_downed
        }
    }
}

impl PaletteRam for Palette {
    fn read(&self, addr: Addr) -> Data {
        //      return this.ram.map((v: Byte, i: number): Byte => {
        //   if (this.isSpriteMirror(i)) return this.ram[i - 0x10];
        //   if (this.isBackgroundMirror(i)) return this.ram[0x00];
        //   return v;
        // });
        10
    }

    fn write(&mut self, addr: Addr, data: Data) {
        let index: usize;
        {
            index = self.get_palette_addr(addr) as usize;
        }
        self.0[index] = data;
    }
}
