use super::BackgroundField;
use super::Tile;

pub fn render(background: &BackgroundField) {
    render_background(background);
}

fn render_background(background: &BackgroundField) {
    for (i, bg) in background.into_iter().enumerate() {
        let x = (i % 33) * 8;
        let y = (i / 33) * 8;
        render_tile(bg, x, y /* , palette */);
    }
}

fn render_tile(bg: &Tile, tile_x: usize, tile_y: usize) {
    let offset_x = 0; // scroll_x % 8;
    let offset_y = 0; // scroll_y % 8;
    // const { data } = this.image;
    let mut data: Vec<u8> = Vec::new();
    for i in 0..8 {
        for j in 0..8 {
            let palette_index = bg.palette_id * 4 + bg.sprite[i][j];
            // let color_id = palette[palette_index];
            // let color = colors[color_id];
            let x = tile_x + j - offset_x;
            let y = tile_y + i - offset_y;
            if x >= 0 as usize && 0xFF >= x && y >= 0 as usize && y < 224 {
                let index = (x + (y * 0x100)) * 4;
                data[index] = if bg.sprite[i][j] == 0 { 0x00 } else { 0xFF }; // color[0];
                data[index + 1] = if bg.sprite[i][j] == 0 { 0x00 } else { 0xFF }; // color[1];
                data[index + 2] = if bg.sprite[i][j] == 0 { 0x00 } else { 0xFF }; // color[2];
                data[index + 3] = 0xFF;
            }
        }
    }
    // for (let i = 0; i < 8; i = (i + 1) | 0) {
    //   for (let j = 0; j < 8; j = (j + 1) | 0) {
    //     const paletteIndex = paletteId * 4 + sprite[i][j];
    //     const colorId = palette[paletteIndex];
    //     const color = colors[colorId];
    //     const x = tileX + j - offsetX;
    //     const y = tileY + i - offsetY;
    //     if (x >= 0 && 0xFF >= x && y >= 0 && y < 224) {
    //       const index = (x + (y * 0x100)) * 4;
    //       data[index] = color[0];
    //       data[index + 1] = color[1];
    //       data[index + 2] = color[2];
    //       data[index + 3] = 0xFF;
    //     }
    //   }
    // }
}