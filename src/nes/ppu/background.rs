use std::cell::Cell;

use super::super::types::Addr;
use super::super::ram::Ram;
use super::tile::Tile;
use super::sprite::*;

#[derive(Debug)]
pub struct Background {
    pub field: Vec<Tile>,
}

const TILE_PER_LINE: u8 = 32;

impl Background {
    pub fn new() -> Self {
        Background { field: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.field = Vec::new();
    }

    pub fn build_line(&mut self,
                      vram: &Ram,
                      cram: &Ram,
                      tile_y: u8,
                      scroll_x: u8,
                      config: &SpriteConfig) {
        // INFO: Horizontal offsets range from 0 to 255. "Normal" vertical offsets range from 0 to 239,
        // while values of 240 to 255 are treated as -16 through -1 in a way, but tile data is incorrectly
        // fetched from the attribute table.
        let clamped_tile_y = tile_y % 30;
        let table_id_offset = ((tile_y / 30) % 2) * 2;
        let scroll_tile_x = scroll_x / 8;
        // background of a line.
        // Build viewport + 1 tile for background scroll.
        for x in 0..(TILE_PER_LINE + 1) {
            let tile_x = x + scroll_tile_x;
            let clamped_tile_x = tile_x % TILE_PER_LINE;
            let name_table_id = ((tile_x / TILE_PER_LINE) % 2) + table_id_offset;
            let offset_add_by_name_table = name_table_id * 0x400;
            //   const tile = this.buildTile(clampedTileX, clampedTileY, offsetAddrByNameTable);
            let position: SpritePosition = (clamped_tile_x as u8, clamped_tile_y as u8);
            self.field.push(Tile::new(vram, cram, &position, config));
        }
    }
}
