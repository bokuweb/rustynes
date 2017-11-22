use super::super::types::{Addr, Data};

#[derive(Debug)]
pub enum PaletteType {
    Sprite,
    Background,
}

pub type PalleteList = Vec<u8>;

#[derive(Debug)]
pub struct Palette(PalleteList);

pub trait PaletteRam {
    fn get(&self, palette_id: u8, palette_type: PaletteType) -> PalleteList;

    fn read(&self, addr: Addr) -> Data;

    fn write(&mut self, addr: Addr, data: Data);
}

impl Palette {
    pub fn new() -> Self {
        Palette(vec!(0; 0x20))
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
    fn get(&self, palette_id: u8, palette_type: PaletteType) -> PalleteList {
         println!("palette_id = {} {:?}", palette_id, self.0);
        let offset = match palette_type {
            PaletteType::Sprite => 0x10,
            _ => 0x00,
        };
        let start = (palette_id * 4 + offset) as usize;
        let end = start + 4;
        (start..end).map(|p| self.0[p]).collect()
    }

    fn read(&self, addr: Addr) -> Data {
        if self.is_sprite_mirror(addr) {
            return self.0[(addr - 0x10) as usize];
        }
        if self.is_background_mirror(addr) {
            return self.0[0x00];
        }
        self.0[addr as usize]
    }

    fn write(&mut self, addr: Addr, data: Data) {
        let index: usize;
        {
            index = self.get_palette_addr(addr) as usize;
        }
        // println!("palette write {:X} {:X} {:?}", index, addr, data);
        self.0[index] = data;
    }
}

#[test]
fn test_get_baclground_palette() {
    let mut p = Palette::new();
    for x in 0..4 {
        p.write(x, x as Data);
    }
    let palette = p.get(0x00, PaletteType::Background);
    assert_eq!(palette.len(), 0x4);
    assert_eq!(palette[0], 0x0);
    assert_eq!(palette[1], 0x1);
    assert_eq!(palette[2], 0x2);
    assert_eq!(palette[3], 0x3);
}

#[test]
fn test_get_sprite_palette() {
    let mut p = Palette::new();
    for x in 0x10..0x14 {
        p.write(x, x as Data);
    }
    let palette = p.get(0x00, PaletteType::Sprite);
    assert_eq!(palette.len(), 0x4);
    assert_eq!(palette[0], 0x0);
    assert_eq!(palette[1], 0x11);
    assert_eq!(palette[2], 0x12);
    assert_eq!(palette[3], 0x13);
}