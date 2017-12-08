mod color;

use super::BackgroundField;
use super::Tile;
use super::PaletteList;
use super::{Sprite, SpritesWithCtx, SpritePosition};
use self::color::COLORS;

extern "C" {
    fn canvas_render(ptr: *const u8, len: usize);
}

pub fn render(background: &BackgroundField, sprites: &SpritesWithCtx) {
    let mut buf: Vec<u8> = vec![0; 256 * 224 * 4];
    render_background(&mut buf, background);
    render_sprites(&mut buf, sprites);
    unsafe {
        canvas_render(buf.as_ptr(), buf.len());
    }
}

fn render_background(buf: &mut Vec<u8>, background: &BackgroundField) {
    for (i, bg) in background.into_iter().enumerate() {
        let x = (i % 33) * 8;
        let y  = (i / 33) * 8;
        render_tile(buf, bg, x, y);
    }
}

fn render_sprites(buf: &mut Vec<u8>, sprites: &SpritesWithCtx) {
    for sprite in sprites {
        render_sprite(buf, &sprite.sprite, &sprite.position, &sprite.palette);
    }
}

fn render_sprite(data: &mut Vec<u8>,
                 sprite: &Sprite,
                 position: &SpritePosition,
                 palette: &PaletteList) {
    for i in 0..8 {
        for j in 0..8 {
            if sprite[i][j] != 0 {
                let color_id = palette[sprite[i][j] as usize];
                let color = COLORS[color_id as usize];
                let x = position.0 as usize + j;
                let y = position.1 as usize + i;
                let index = (x + (y * 0x100)) * 4;
                data[index] = color.0;
                data[index + 1] = color.1;
                data[index + 2] = color.2;
                data[index + 3] = 0xFF;
            }
        }
    }
}


fn render_tile(data: &mut Vec<u8>, bg: &Tile, x: usize, y: usize) {
    let offset_x = 0; // TODO: scroll_x % 8;
    let offset_y = 0; // TODO: scroll_y % 8;
    for i in 0..8 {
        for j in 0..8 {
            let color_id = bg.palette[bg.sprite[i][j] as usize];
            let color = COLORS[color_id as usize];
            let x = x + j - offset_x;
            let y = y + i - offset_y;
            if x >= 0 as usize && 0xFF >= x && y >= 0 as usize && y < 224 {
                let index = (x + (y * 0x100)) * 4;
                data[index] = color.0;
                data[index + 1] = color.1;
                data[index + 2] = color.2;
                data[index + 3] = 0xFF;
            }
        }
    }
}