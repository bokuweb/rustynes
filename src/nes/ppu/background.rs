use std::cell::Cell;

use super::super::types::Addr;
use super::super::ram::Ram;
use super::tile::Tile;
use super::sprite_helper::*;

#[derive(Debug)]
pub struct Background {
    pub field: Vec<Tile>,
}

impl Background {
    pub fn new() -> Self {
        Background { field: vec![] }
    }

    pub fn clear(&mut self) {
        self.field = vec![];
    }

    pub fn build(&self, vram: &Ram, cram: &Ram, position: &SpritePosition, config: &SpriteConfig) {}
}
