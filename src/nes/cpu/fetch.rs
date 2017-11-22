use super::opecode::*;
use super::super::cpu_registers::CpuRegisters;
use super::super::bus::cpu_bus::CpuBus;
use super::super::types::{Data, Addr, Word};

pub fn fetch<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Data {
    let code = bus.read(registers.get_PC());
    registers.inc_PC();
    code
}

pub fn fetch_opeland<T: CpuRegisters, U: CpuBus>(code: &Opecode,
                                                 registers: &mut T,
                                                 bus: &mut U)
                                                 -> Word {
    match code.mode {
        Addressing::Accumulator => 0x0000,
        Addressing::Implied => 0x0000,
        Addressing::Immediate => fetch(registers, bus) as Word,
        Addressing::Relative => fetch_relative(registers, bus),
        Addressing::ZeroPage => fetch(registers, bus) as Word,
        Addressing::ZeroPageX => fetch_zeropage_x(registers, bus),
        Addressing::ZeroPageY => fetch_zeropage_y(registers, bus),
        Addressing::Absolute => fetch_word(registers, bus),     
        Addressing::AbsoluteX => fetch_absolute_x(registers, bus),
        Addressing::AbsoluteY => fetch_absolute_y(registers, bus),
        Addressing::PreIndexedIndirect => fetch_pre_indexed_indirect(registers, bus),
        Addressing::PostIndexedIndirect => fetch_post_indexed_indirect(registers, bus),
        Addressing::IndirectAbsolute => fetch_indirect_absolute(registers, bus),
    }
}

pub fn fetch_word<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Word {
    let lower = bus.read(registers.get_PC()) as Word;
    registers.inc_PC();
    let upper = bus.read(registers.get_PC()) as Word;
    registers.inc_PC();
    (upper << 8 | lower) as Word
}

pub fn fetch_relative<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Word {
    let base = fetch(registers, bus) as Word;
    if base < 0x80 {
        base + registers.get_PC()
    } else {
        base + registers.get_PC() - 256
    }
}

pub fn fetch_zeropage_x<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Word {
    let addr = fetch(registers, bus) as Word;
    (addr + registers.get_X() as Word) & 0xFF as Word
}

pub fn fetch_zeropage_y<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Word {
    let addr = fetch(registers, bus) as Word;
    (addr + registers.get_Y() as Word) & 0xFF as Word
}

pub fn fetch_absolute_x<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Word {
    let addr = fetch_word(registers, bus);
    (addr + registers.get_X() as Word) & 0xFFFF
}

pub fn fetch_absolute_y<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Word {
    let addr = fetch_word(registers, bus);
    (addr + registers.get_Y() as Word) & 0xFFFF
}

pub fn fetch_pre_indexed_indirect<T: CpuRegisters, U: CpuBus>(registers: &mut T,
                                                              bus: &mut U)
                                                              -> Word {
    let addr = ((fetch(registers, bus) + registers.get_X()) & 0xFF) as Addr;
    let addr = (bus.read(addr) as Addr) + ((bus.read((addr + 1) as Addr & 0xFF) as Addr) << 8);
    addr & 0xFFFF
}

pub fn fetch_post_indexed_indirect<T: CpuRegisters, U: CpuBus>(registers: &mut T,
                                                               bus: &mut U)
                                                               -> Word {
    let addr = fetch(registers, bus) as Addr;
    let base_addr = (bus.read(addr) as usize) + ((bus.read((addr + 1) & 0x00FF) as usize) * 0x100);
    ((base_addr + (registers.get_Y() as usize)) & 0xFFFF) as u16
}

pub fn fetch_indirect_absolute<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Word {
    let addr = fetch_word(registers, bus);
    let upper = bus.read((addr & 0xFF00) | ((((addr & 0xFF) + 1) & 0xFF)) as Addr) as Addr;
    let addr = (bus.read(addr) as Addr) + (upper << 8) as Addr;
    addr & 0xFFFF
}
