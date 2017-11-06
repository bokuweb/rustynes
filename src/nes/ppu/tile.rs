use self::super::Ram;
use self::super::sprite_helper::{Sprite, SpritePosition, SpriteConfig, build, get_attribute,
                                 get_block_id, get_sprite_id};

pub struct Tile {
    sprite: Sprite,
    palette_id: u8,
}

pub struct TileParams {
    position: SpritePosition,
    config: SpriteConfig,
}

impl Tile {
    fn new(vram: &Ram, cram: &Ram, params: &TileParams) -> Self {
        // INFO see. http://hp.vector.co.jp/authors/VA042397/nes/ppu.html
        let position = &params.position;
        let config = &params.config;
        let block_id = get_block_id(position);
        let sprite_id = get_sprite_id(&vram, position, config);
        let attr = get_attribute(&vram, position, config);
        let palette_id = (attr >> (block_id * 2)) & 0x03;
        let sprite = build(&cram, sprite_id, config);
        Tile { sprite, palette_id }
    }
}

#[test]
fn get_block_id() {
    let id = Tile::get_block_id(2, 3);
    assert_eq!(id, 3);
}
