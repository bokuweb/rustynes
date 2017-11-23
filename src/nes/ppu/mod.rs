
pub mod background;
pub mod tile;
mod sprite;
mod registers;
mod palette;

use self::super::ram::Ram;
use self::sprite::*;
use self::registers::*;
use super::types::{Data, Addr};
pub use self::background::*;
pub use self::tile::*;
pub use self::palette::*;


#[derive(Debug)]
pub struct PpuConfig {
    pub is_horizontal_mirror: bool,
}

#[derive(Debug)]
pub struct PpuCtx<P: PaletteRam> {
    pub palette: P,
    pub vram: Box<Ram>,
    pub cram: Box<Ram>,
    pub sprite_ram: Box<Ram>,
}

const CYCLES_PER_LINE: usize = 341;

#[derive(Debug)]
pub struct Ppu {
    pub cycle: usize,
    pub line: usize,
    pub registers: Registers,
    pub ctx: PpuCtx<Palette>,
    pub background: Background,
    pub config: PpuConfig,
}

impl Ppu {
    pub fn new(character_ram: Vec<u8>, config: PpuConfig) -> Ppu {
        Ppu {
            cycle: 0,
            line: 0,
            registers: Registers::new(),
            ctx: PpuCtx {
                palette: Palette::new(),
                vram: Box::new(Ram::new(vec![0; 0x2000])),
                cram: Box::new(Ram::new(character_ram)),
                sprite_ram: Box::new(Ram::new(vec![0; 0x0100])),
            },
            background: Background::new(),
            config,
        }
    }

    pub fn read(&mut self, addr: Addr) -> Data {
        self.registers.read(addr, &mut self.ctx)
    }

    pub fn write(&mut self, addr: Addr, data: Data) {
        println!("[ppu write] addr = {:X}, data = {:X}", addr, data);
        self.registers.write(addr, data, &mut self.ctx);
    }

    // The PPU draws one line at 341 clocks and prepares for the next line.
    // While drawing the BG and sprite at the first 256 clocks,
    // it searches for sprites to be drawn on the next scan line.
    // Get the pattern of the sprite searched with the remaining clock.
    pub fn run(&mut self, cycle: usize) -> bool {
        let cycle = self.cycle + cycle;
        let line = self.line;
        if line == 0 {
            self.background.clear();
            // buildSprites();
        }
        if cycle < CYCLES_PER_LINE {
            self.cycle = cycle;
            return false;
        }
        self.cycle = cycle - CYCLES_PER_LINE;
        self.line = line + 1;

        // if self.hasSpriteHit() {
        //     self.setSpriteHit();
        // }

        if line <= 240 && line % 8 == 0
        /* && self.scrollY <= 240 */
        {
            let mut config = SpriteConfig {
                offset_addr_by_name_table: 0, //TODO: (~~(tileX / 32) % 2) + tableIdOffset;
                offset_addr_by_background_table: 0, // TODO: (registers[0] & 0x10) ? 0x1000 : 0x0000;
                is_horizontal_mirror: self.config.is_horizontal_mirror,
            };
            let tile_y = (line / 8) as u8; // TODO: + scroll_y;
            let scroll_x = 0;
            self.background
                .build_line(&self.ctx.vram,
                            &self.ctx.cram,
                            &self.ctx.palette,
                            tile_y,
                            scroll_x,
                            &mut config);
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
            // println!("{:?}", self.vram);
            //return Some(RenderingContext { background: self.background });
            //   background: this.isBackgroundEnable ? this.background : null,
            //   sprites: this.isSpriteEnable ? this.sprites : null,
            //   palette: this.getPalette(),
            // };
            self.line = 0;
            return true;
        }
        false
    }

    // fn get_scroll_tile_y(&self) -> u8 {
    //     // self.registers.scroll_y + ((self.registers.name_table_id / 2) * 240)) / 8);
    //     0
    // }
    //
    // fn get_tile_y(&self) -> u8 {
    //     (self.line / 8) as u8 + self.get_scroll_tile_y()
    // }
}
