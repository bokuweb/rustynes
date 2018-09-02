use super::super::types::Addr;
use super::super::Ram;
use super::super::Mmc;

pub type Sprite = Vec<Vec<u8>>;

pub type SpritePosition = (u8, u8);

#[derive(Debug)]
pub struct SpriteConfig {
    pub offset_addr_by_name_table: Option<u16>,
    pub offset_addr_by_background_table: u16,
    pub offset_addr_by_sprite_table: u16,
    pub is_horizontal_mirror: bool,
    pub is_background_enable: bool,
}

pub fn mirror_down_sprite_addr(addr: Addr, is_horizontal_mirror: bool) -> Addr {
    if !is_horizontal_mirror {
        return addr;
    }
    if (addr >= 0x0400 && addr < 0x0800) || addr >= 0x0C00 {
        return addr - 0x400 as Addr;
    }
    addr
}

pub fn get_block_id(position: &SpritePosition) -> u8 {
    ((position.0 % 4) / 2) + (((position.1 % 4) / 2) * 2)
}

pub fn get_sprite_id(vram: &Ram, position: &SpritePosition, config: &SpriteConfig) -> u8 {
    let tile_number = position.1 as Addr * 32 + position.0 as Addr;
    let addr = tile_number + config.offset_addr_by_name_table.unwrap();
    let addr = mirror_down_sprite_addr(addr, config.is_horizontal_mirror);
    let data = vram.read(addr);
    data
}

pub fn get_attribute(vram: &Ram, position: &SpritePosition, config: &SpriteConfig) -> u8 {
    let addr = 0x03C0 + ((position.0 / 4) + ((position.1 / 4) * 8)) as u16 +
               config.offset_addr_by_name_table.unwrap();
    vram.read(mirror_down_sprite_addr(addr, config.is_horizontal_mirror))
}

pub fn build(cram: &Ram, sprite_id: u8, offset: u16, mmc: &Mmc, is_8x8: bool) -> Sprite {
    let h = if is_8x8 { 1 } else { 2 };
    let mut sprite: Sprite = (0..8 * h).into_iter().map(|_| vec![0; 8 * h]).collect();
    for k in 0..h {
        for i in 0..16 {
            for j in 0..8 {
                let addr = ((sprite_id + (k as u8)) as u16) * 16 + i + offset;
                let ram = cram.read(mmc.create_chram_addr(addr));
                if ram & (0x80 >> j) as u8 != 0 {
                    sprite[((k as u16) * 8 + i % 8) as usize][j] += (0x01 << (i / 8)) as u8;
                }
            }
        }
    }
    sprite
}

#[test]
fn test_get_block_id() {
    let position = (2, 3);
    let id = get_block_id(&position);
    assert_eq!(id, 3);
}
