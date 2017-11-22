mod ppu_addr;
mod ppu_data;

use super::super::types::{Data, Addr, Word};
use super::super::Ram;
use super::palette::*;
// use super::super::helper::*;
use self::ppu_addr::PpuAddr;
use self::ppu_data::PpuData;


#[derive(Debug)]
pub struct Registers {
    ppu_addr: PpuAddr,
    ppu_data: PpuData,
}

// PPU power up state
  // see. https://wiki.nesdev.com/w/index.php/PPU_power_up_state
  //
  // Memory map
  /*
  | addr           |  description               |
  +----------------+----------------------------+
  | 0x0000-0x0FFF  |  Pattern table#0           |
  | 0x1000-0x1FFF  |  Pattern table#1           |
  | 0x2000-0x23BF  |  Name table                |
  | 0x23C0-0x23FF  |  Attribute table           |
  | 0x2400-0x27BF  |  Name table                |
  | 0x27C0-0x27FF  |  Attribute table           |
  | 0x2800-0x2BBF  |  Name table                |
  | 0x2BC0-0x2BFF  |  Attribute table           |
  | 0x2C00-0x2FBF  |  Name Table                |
  | 0x2FC0-0x2FFF  |  Attribute Table           |
  | 0x3000-0x3EFF  |  mirror of 0x2000-0x2EFF   |
  | 0x3F00-0x3F0F  |  background Palette        |
  | 0x3F10-0x3F1F  |  sprite Palette            |
  | 0x3F20-0x3FFF  |  mirror of 0x3F00-0x3F1F   |
  */







/*
    Control Register1 0x2000
  | bit  | description                                 |
  +------+---------------------------------------------+
  |  7   | Assert NMI when VBlank 0: disable, 1:enable |
  |  6   | PPU master/slave, always 1                  |
  |  5   | Sprite size 0: 8x8, 1: 8x16                 |
  |  4   | Bg pattern table 0:0x0000, 1:0x1000         |
  |  3   | sprite pattern table 0:0x0000, 1:0x1000     |
  |  2   | PPU memory increment 0: +=1, 1:+=32         |
  |  1-0 | Name table 0x00: 0x2000                     |
  |      |            0x01: 0x2400                     |
  |      |            0x02: 0x2800                     |
  |      |            0x03: 0x2C00                     |
  */

/*
    Control Register2 0x2001
  | bit  | description                                 |
  +------+---------------------------------------------+
  |  7-5 | Background color  0x00: Black               |
  |      |                   0x01: Green               |
  |      |                   0x02: Blue                |
  |      |                   0x04: Red                 |
  |  4   | Enable sprite                               |
  |  3   | Enable background                           |
  |  2   | Sprite mask       render left end           |
  |  1   | Background mask   render left end           |
  |  0   | Display type      0: color, 1: mono         |
  */


pub trait PpuRegisters {
    fn read<P: PaletteRam>(&mut self, addr: Addr, vram: &Ram, cram: &Ram, palette: &P) -> Data;

    fn write<P: PaletteRam>(&mut self, addr: Addr, data: Data, vram: &Ram, cram: &Ram, palette: &mut P);

    fn write_ppu_addr(&mut self, data: Data);

    fn read_ppu_data<P: PaletteRam>(&mut self, vram: &Ram, cram: &Ram, palette: &P) -> Data;

    fn write_ppu_data<P: PaletteRam>(&mut self, vram: &Ram, cram: &Ram, data: Data, palette: &mut P);
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            ppu_addr: PpuAddr::new(),
            ppu_data: PpuData::new(),
        }
    }
}

impl PpuRegisters for Registers {
    fn read<P: PaletteRam>(&mut self, addr: Addr, vram: &Ram, cram: &Ram, palette: &P) -> Data {
        match addr {
            0x0002 => {
                //this.isHorizontalScroll = true;
                //const data = this.registers[0x02];
                // this.clearVblank();
                // this.clearSpriteHit();
                // return data;
                return 0;
            }
            0x0004 => {
                // return this.spriteRam.read(this.spriteRamAddr);
                return 0;
            }
            0x0007 => self.read_ppu_data(vram, cram, palette),
            _ => 0,
        }
    }

    fn write<P: PaletteRam>(&mut self, addr: Addr, data: Data, vram: &Ram, cram: &Ram, palette: &mut P) {
        match addr {
            0x0006 => self.write_ppu_addr(data),
            0x0007 => self.write_ppu_data(vram, cram, data, palette),
            _ => (),
        }
    }

    fn write_ppu_addr(&mut self, data: Data) {
        self.ppu_addr.write(data);
    }

    fn read_ppu_data<P: PaletteRam>(&mut self, vram: &Ram, cram: &Ram, palette: &P) -> Data {
        let addr = self.ppu_addr.get();
        let data = self.ppu_data.read(vram, cram, addr, palette);
        self.ppu_addr.update(0x01); // TODO: update 1 or 32
        data
    }

    fn write_ppu_data<P: PaletteRam>(&mut self, vram: &Ram, cram: &Ram, data: Data, palette: &mut P) {
        let addr = self.ppu_addr.get();
        self.ppu_data.write(vram, cram, addr, data, palette);
        self.ppu_addr.update(0x01); // TODO: update 1 or 32
    }
}