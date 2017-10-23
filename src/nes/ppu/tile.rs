use std::cell::Cell;

use super::super::types::Addr;

#[derive(Debug)]
pub struct Tile {}

fn mirror_down_sprite_addr(addr: Addr) -> Addr {
    // if (!this.config.isHorizontalMirror) return addr;
    if addr >= 0x0400 && addr < 0x0800 || addr >= 0x0C00 {
        return addr - 0x400 as Addr;
    }
    addr
}

fn get_block_id(x: u8, y: u8) -> u8 {
    ((x % 4) / 2) + (((y % 4) / 2) * 2)
}

fn get_sprite_id(x: u8, y: u8, offset: u16) -> u8 {
    let tile_number = y as Addr * 32 + x as Addr;
    let addr = mirror_down_sprite_addr(tile_number + offset);
    // return this.vram.read(spriteAddr);
    0
}

fn get_attribute(x: u8, y: u8, offset: u16) -> u8 {
    let addr = 0x03C0 + ((x / 4) + ((y / 4) * 8)) as u16 + offset;
    0
    // return this.vram.read(this.mirrorDownSpriteAddr(addr));
}

fn build(sprite_id: u8, offset: u16) -> Vec<Vec<u8>> {
    let mut sprite: Vec<Vec<u8>> = (0..8).into_iter().map(|_| vec![0; 8]).collect();
    for i in 0..16 {
        for j in 0..8 {
            let addr = (sprite_id as u16) * 16 + i + offset;
            let ram = 0 as u8; // this.readCharacterRAM(addr);
            if ram & (0x80 >> j) as u8 != 0 {
                sprite[(i % 8) as usize][j] += (0x01 << (i / 8)) as u8;
            }
        }
    }
    sprite
}

impl Tile {
    pub fn new() -> Self {
        Tile {}
    }

    fn build(&self, x: u8, y: u8, offset: u16) -> Tile {
        // INFO see. http://hp.vector.co.jp/authors/VA042397/nes/ppu.html
        let block_id = get_block_id(x, y);
        let sprite_id = get_sprite_id(x, y, offset);
        let attr = get_attribute(x, y, offset);
        let palette_id = (attr >> (block_id * 2)) & 0x03;
        let sprite = build(sprite_id, 0 /* ,this.backgroundTableOffset */);
        // return {
        //   sprite,
        //   paletteId,
        // };
        Tile {}
    }
}


#[test]
fn get_block_id() {
    let id = Tile::get_block_id(2, 3);
    assert_eq!(id, 3);
}
