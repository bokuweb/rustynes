const SPRITES_NUMBER: u16 = 0x100;

// use self::super::Ram;
// use self::super::sprite_utils::{Sprite, SpritePosition, SpriteConfig, build, get_attribute,
//                                 get_block_id, get_sprite_id};
// use self::super::palette::*;
//
pub fn build_sprites<P: PaletteRam>(sprite_ram: &Ram, palette: &P, config: &SpriteConfig) {
    // const offset = (this.registers[0] & 0x08) ? 0x1000 : 0x0000;
    for x in 0..(SPRITES_NUMBER / 4) {
        // INFO: Offset sprite Y position, because First and last 8line is not rendered.
        let base = i * 4;
        let y = sprites_ram.read(base) - 8;
        if y >= 0 {
            let sprite_id = sprite_ram.read(base + 1);
            let attr = sprite_ram.read(base + 2);
            let x = sprite_ram.read(base + 3);
            //   const sprite = this.buildSprite(spriteId, offset);
            //   this.sprites[i / 4] = { sprite, x, y, attr, spriteId };
        }
    }


}
