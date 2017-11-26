use self::super::Ram;
use self::super::sprite_utils::*;
use self::super::palette::*;

#[derive(Debug)]
pub struct Tile {
    pub sprite: Sprite,
    pub palette: PaletteList,
}

impl Tile {
    pub fn new<P: PaletteRam>(vram: &Ram, cram: &Ram, palette: &P, position: &SpritePosition, config: &SpriteConfig) -> Self {
        // INFO see. http://hp.vector.co.jp/authors/VA042397/nes/ppu.html
        let block_id = get_block_id(position);
        let sprite_id = get_sprite_id(&vram, position, config);
        let attr = get_attribute(&vram, position, config);
        let palette_id = (attr >> (block_id * 2)) & 0x03;
        let sprite = build(&cram, sprite_id, config.offset_addr_by_background_table);
        Tile { sprite, palette: palette.get(palette_id, PaletteType::Background) }
    }
}
