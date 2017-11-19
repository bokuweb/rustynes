use self::super::Ram;
use self::super::sprite::{Sprite, SpritePosition, SpriteConfig, build, get_attribute,
                                 get_block_id, get_sprite_id};
#[derive(Debug)]
pub struct Tile {
    pub sprite: Sprite,
    pub palette_id: u8,
}

impl Tile {
    pub fn new(vram: &Ram, cram: &Ram, position: &SpritePosition, config: &SpriteConfig) -> Self {
        // INFO see. http://hp.vector.co.jp/authors/VA042397/nes/ppu.html
        let block_id = get_block_id(position);
        let sprite_id = get_sprite_id(&vram, position, config);
        let attr = get_attribute(&vram, position, config);
        let palette_id = (attr >> (block_id * 2)) & 0x03;
        let sprite = build(&cram, sprite_id, config);
        Tile { sprite, palette_id }
    }
}

