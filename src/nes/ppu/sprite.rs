use self::super::sprite_utils::*;
use self::super::Ram;
use self::super::palette::*;

const SPRITES_NUMBER: u16 = 0x100;

pub type SpritesWithCtx = Vec<SpriteWithCtx>;

#[derive(Debug)]
pub struct SpriteWithCtx {
    pub sprite: Sprite,
    pub position: SpritePosition,
    pub attr: u8,
    pub palette: PaletteList,
}

pub fn build_sprites<P: PaletteRam>(buf: &mut SpritesWithCtx,
                                    cram: &Ram,
                                    sprite_ram: &Ram,
                                    palette: &P,
                                    offset: u16) {
    // let mut sprites = Vec::new();
    for i in 0..(SPRITES_NUMBER / 4) {
        // INFO: Offset sprite Y position, because First and last 8line is not rendered.
        let base = i * 4;
        let y = sprite_ram.read(base) as i8 - 8;
        // println!("y = {}", y);
        if y >= 0 as i8 {
            let sprite_id = sprite_ram.read(base + 1);
            let attr = sprite_ram.read(base + 2);
            let x = sprite_ram.read(base + 3);
            let sprite = build(&cram, sprite_id, offset);
            let position: SpritePosition = (x, y as u8);
            let palette_id = attr & 0x03;
            buf.push(SpriteWithCtx {
                             sprite,
                             position,
                             attr,
                             palette: palette.get(palette_id, PaletteType::Sprite),
                         });
        }
    }
}
