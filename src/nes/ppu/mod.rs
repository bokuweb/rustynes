mod tile;
mod sprite_helper;

use self::super::ram::Ram;
use std::cell::Cell;
use sprite_helper::*;

const CYCLES_PER_LINE: usize = 341;

#[derive(Debug)]
pub struct Ppu {
    cycle: Cell<usize>,
    line: Cell<usize>,
    vram: Ram,
    cram: Ram,
}

pub struct RenderingContext {}

impl Ppu {
    pub fn new(character_ram: Vec<u8>) -> Ppu {
        Ppu {
            cycle: Cell::new(0),
            line: Cell::new(0),
            vram: Ram::new(vec![0; 0x2000]),
            cram: Ram::new(character_ram),
        }
    }

    // The PPU draws one line at 341 clocks and prepares for the next line.
    // While drawing the BG and sprite at the first 256 clocks,
    // it searches for sprites to be drawn on the next scan line.
    // Get the pattern of the sprite searched with the remaining clock.
    pub fn run(&self, cycle: usize) -> Option<RenderingContext> {
        let mut cycle = self.cycle.get() + cycle;
        let line = self.line.get();
        if line == 0 {
            // this.background.length = 0;
            // this.buildSprites();
        }
        if cycle < CYCLES_PER_LINE {
            self.cycle.set(cycle);
            return None;
        }
        self.cycle.set(cycle - CYCLES_PER_LINE);
        self.line.set(line + 1);

        // if self.hasSpriteHit() {
        //     self.setSpriteHit();
        // }

        if line <= 240 && line % 8 == 0
        /* && self.scrollY <= 240 */
        {
            // this.buildBackground();
        }

        if line == 241 {
            // self.setVblank();
            // if (this.hasVblankIrqEnabled) {
            //   this.interrupts.assertNmi();
            // }
        }

        if line == 262 {
            // this.clearVblank();
            // this.clearSpriteHit();
            // this.line = 0;
            // this.interrupts.deassertNmi();
            return Some(RenderingContext {});
            //   background: this.isBackgroundEnable ? this.background : null,
            //   sprites: this.isSpriteEnable ? this.sprites : null,
            //   palette: this.getPalette(),
            // };
        }
        None
    }
}
