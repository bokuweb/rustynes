mod color;

use super::BackgroundField;
use super::Tile;
use super::SpriteWithCtx;
use self::color::COLORS;

extern "C" {
    fn canvas_render(ptr: *const u8, len: usize);
}

pub fn render(background: &BackgroundField, sprites: &Vec<SpriteWithCtx>) {
    render_background(background);
}

fn render_background(background: &BackgroundField) {
    let mut data: Vec<u8> = vec![0; 256 * 224 * 4];
    for (i, bg) in background.into_iter().enumerate() {
        let x = (i % 33) * 8;
        let y = (i / 33) * 8;
        render_tile(&mut data, bg, x, y /* , palette */);
    }
    unsafe {
        canvas_render(data.as_ptr(), data.len());
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