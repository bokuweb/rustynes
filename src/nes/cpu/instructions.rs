use super::super::cpu_registers::CpuRegisters;
use super::super::bus::cpu_bus::CpuBus;
use super::super::types::{Data, Addr, Word};

pub fn sta(opeland: Word, ref mut registers: &mut CpuRegisters, ref mut bus: &mut CpuBus) {
    bus.write(opeland, registers.get_A());
}
