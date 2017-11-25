use self::super::sprite_utils::*;
use self::super::Ram;
use self::super::palette::*;

const SPRITES_NUMBER: u16 = 0x100;

#[derive(Debug)]
pub struct SpriteWithCtx {
    sprite: Sprite,
    position: SpritePosition,
    attr: u8,
    palette: PalleteList,
}

pub fn build_sprites<P: PaletteRam>(cram: &Ram,
                                    sprite_ram: &Ram,
                                    palette: &P,
                                    offset: u16)
                                    -> Vec<SpriteWithCtx> {
    let mut sprites = Vec::new();
    for i in 0..(SPRITES_NUMBER / 4) {
        // INFO: Offset sprite Y position, because First and last 8line is not rendered.
        let base = i * 4;
        let y = sprite_ram.read(base) as i8 - 8;
        if y >= 0 as i8 {
            let sprite_id = sprite_ram.read(base + 1);
            let attr = sprite_ram.read(base + 2);
            let x = sprite_ram.read(base + 3);
            let sprite = build(&cram, sprite_id, offset);
            let position: SpritePosition = (x, y as u8);
            let palette_id = attr & 0x03;
            sprites.push(SpriteWithCtx {
                             sprite,
                             position,
                             attr,
                             palette: palette.get(palette_id, PaletteType::Sprite),
                         });
        }
    }
    sprites
}
