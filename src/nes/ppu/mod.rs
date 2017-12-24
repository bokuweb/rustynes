
pub mod background;
pub mod tile;
mod sprite_utils;
mod sprite;
mod registers;
mod palette;

use self::super::ram::Ram;
use self::registers::*;
use super::types::{Data, Addr};
pub use self::background::*;
pub use self::tile::*;
pub use self::palette::*;
pub use self::sprite::*;
pub use self::sprite_utils::*;

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
    pub sprites: SpritesWithCtx,
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
            sprites: Vec::new(),
            background: Background::new(),
            config,
        }
    }

    pub fn read(&mut self, addr: Addr) -> Data {
        self.registers.read(addr, &mut self.ctx)
    }

    pub fn write(&mut self, addr: Addr, data: Data) {
        // println!("ppu addr = {:X}, data = {:X}", addr, data);
        self.registers.write(addr, data, &mut self.ctx);
    }

    // The PPU draws one line at 341 clocks and prepares for the next line.
    // While drawing the BG and sprite at the first 256 clocks,
    // it searches for sprites to be drawn on the next scan line.
    // Get the pattern of the sprite searched with the remaining clock.
    pub fn run(&mut self, cycle: usize, nmi: &mut bool) -> bool {
        let cycle = self.cycle + cycle;
        let line = self.line;
        if line == 0 {
            self.background.clear();
            self.sprites = Vec::new();
            build_sprites(&mut self.sprites,
                          &self.ctx.cram,
                          &self.ctx.sprite_ram,
                          &self.ctx.palette,
                          self.registers.get_sprite_table_offset());
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
                offset_addr_by_sprite_table: self.registers.get_sprite_table_offset(), // TODO: (this.registers[0] & 0x08) ? 0x1000 : 0x0000;
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
            self.registers.set_vblank();
            if self.registers.is_irq_enable() {
                *nmi = true;
            }
        }

        if line == 262 {
            self.registers.clear_vblank();
            self.registers.clear_sprite_hit();
            *nmi = false;
            self.line = 0;
            return true;
        }
        false
    }

    pub fn transfer_sprite(&mut self, addr: Addr, data: Data) {
        let addr = addr + self.registers.oam.get_addr();
        self.ctx.sprite_ram.write(addr % 0x100, data);
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
