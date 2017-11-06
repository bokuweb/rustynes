use super::super::types::Addr;
use super::super::Ram;

pub type Sprite = Vec<Vec<u8>>;

pub type SpritePosition = (u8, u8);

#[derive(Debug)]
pub struct SpriteConfig {
    name_table_offset: u16,
    background_table_offset: u16,
    is_horizontal_mirror: bool,
}

pub fn mirror_down_sprite_addr(addr: Addr, is_horizontal_mirror: bool) -> Addr {
    if is_horizontal_mirror {
        return addr;
    }
    if addr >= 0x0400 && addr < 0x0800 || addr >= 0x0C00 {
        return addr - 0x400 as Addr;
    }
    addr
}

pub fn get_block_id(position: &SpritePosition) -> u8 {
    ((position.0 % 4) / 2) + (((position.1 % 4) / 2) * 2)
}

pub fn get_sprite_id(vram: &Ram, position: &SpritePosition, config: &SpriteConfig) -> u8 {
    let tile_number = position.1 as Addr * 32 + position.0 as Addr;
    let addr = mirror_down_sprite_addr(tile_number + config.name_table_offset,
                                       config.is_horizontal_mirror);
    vram.read(addr)
}

pub fn get_attribute(vram: &Ram, position: &SpritePosition, config: &SpriteConfig) -> u8 {
    let addr = 0x03C0 + ((position.0 / 4) + ((position.1 / 4) * 8)) as u16 +
               config.name_table_offset;
    vram.read(mirror_down_sprite_addr(addr, config.is_horizontal_mirror))
}

pub fn build(cram: &Ram, sprite_id: u8, config: &SpriteConfig) -> Sprite {
    let mut sprite: Sprite = (0..8).into_iter().map(|_| vec![0; 8]).collect();
    for i in 0..16 {
        for j in 0..8 {
            let addr = (sprite_id as u16) * 16 + i + config.background_table_offset;
            let ram = cram.read(addr);
            if ram & (0x80 >> j) as u8 != 0 {
                sprite[(i % 8) as usize][j] += (0x01 << (i / 8)) as u8;
            }
        }
    }
    sprite
}
