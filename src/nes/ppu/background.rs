// use std::cell::Cell;

use super::super::types::{Addr, Data};
use super::super::ram::Ram;
use super::tile::Tile;
use super::sprite_utils::*;
use super::palette::*;

#[derive(Debug,)]
pub struct BackgroundUnit {
    pub tile: Tile,
    pub scroll_x: Data,
    pub scroll_y: Data,
}

pub type BackgroundField = Vec<BackgroundUnit>;

#[derive(Debug,)]
pub struct Background(pub BackgroundField);

const TILE_PER_LINE: u8 = 32;

impl Background {
    pub fn new() -> Self {
        Background(Vec::new())
    }

    pub fn clear(&mut self) {
        self.0 = Vec::new();
    }

    pub fn build_line<P: PaletteRam>(&mut self,
                      vram: &Ram,
                      cram: &Ram,
                      palette: &P,
                      tile_y: u8,
                      scroll_x: u8,
                      scroll_y: u8,
                      config: &mut SpriteConfig) {
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
            config.offset_addr_by_name_table = (name_table_id as Addr) * 0x400;
            let position: SpritePosition = (clamped_tile_x as u8, clamped_tile_y as u8);
            self.0.push(
                BackgroundUnit {
                    tile: Tile::new(vram, cram, palette, &position, &config),
                    scroll_x,
                    scroll_y,
                });
        }
    }
}
