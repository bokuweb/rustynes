use std::cell::Cell;

use super::super::types::Addr;
use super::tile::Tile;

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

    pub fn build(&self) {}
}
