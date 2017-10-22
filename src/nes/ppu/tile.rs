use std::cell::Cell;
use super::types::Addr;

#[derive(Debug)]
pub struct Sprite {}

impl Tile {
    pub fn new() -> Self {
        Tile {}
    }

    fn mirror_down_sprite_addr(addr: Addr) -> Addr {
        // if (!this.config.isHorizontalMirror) return addr;
        if addr >= 0x0400 && addr < 0x0800 || addr >= 0x0C00 {
            return addr -= 0x400;
        }
        addr
    }

    fn get_block_id(x: u8, y: u8) -> u8 {
        ((x % 4) / 2) + (((y % 4) / 2) * 2)
    }

    fn get_sprite_id(x: u8, y: u8, offset: u16) -> u8 {
        let tile_number = y as usize * 32 + x;
        let addr = Tile::mirror_down_sprite_addr(tile_number + offset);
        // return this.vram.read(spriteAddr);
        0
    }

    fn build(&self, x: u8, y: u8, offset: u16) -> Tile {
        // INFO see. http://hp.vector.co.jp/authors/VA042397/nes/ppu.html
        let block_id = Tile::get_block_id(x, y);
        let sprite_id = Tile::get_sprite_id(x, y, offset);
        // let attr = this.getAttribute(tileX, tileY, offset);
        // let paletteId = (attr >> (blockId * 2)) & 0x03;
        // let sprite = this.buildSprite(spriteId, this.backgroundTableOffset);
        // return {
        //   sprite,
        //   paletteId,
        //   scrollX: this.scrollX,
        //   scrollY: this.scrollY,
        // };
        Sprite {}
    }
}


#[test]
fn get_block_id() {
    let id = Sprite::get_block_id(2, 3);
    assert_eq!(id, 3);
}
