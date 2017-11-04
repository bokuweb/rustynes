mod opecode;
mod fetch;
mod instructions;

use self::opecode::*;
use self::fetch::*;
use self::instructions::*;

use super::cpu_registers::CpuRegisters;
use super::bus::cpu_bus::CpuBus;
use super::types::Data;

pub fn reset<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &U) {
    let pc = bus.read_word(0xFFFC);
    registers.set_PC(pc);
}

pub fn run<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Data {
    // println!("[registers] {:?}", registers);
    let code = fetch(registers, bus);
    let ref map = opecode::MAP;
    let code = &*map.get(&code).unwrap();
    let opeland = fetch_opeland(&code, registers, bus);
    match code.name {
        Instruction::LDA if code.mode == Addressing::Immediate => lda_imm(opeland, registers),
        Instruction::LDA => lda(opeland, registers, bus),
        Instruction::LDX if code.mode == Addressing::Immediate => ldx_imm(opeland, registers),
        Instruction::LDX => ldx(opeland, registers, bus),
        Instruction::LDY if code.mode == Addressing::Immediate => ldy_imm(opeland, registers),
        Instruction::LDY => ldy(opeland, registers, bus),
        Instruction::STA => sta(opeland, registers, bus),
        Instruction::STX => stx(opeland, registers, bus),
        Instruction::STY => sty(opeland, registers, bus),
        Instruction::TXA => txa(registers),
        Instruction::TYA => tya(registers),
        Instruction::TXS => txs(registers),
        Instruction::TAY => tay(registers),
        Instruction::TAX => tax(registers),
        Instruction::TSX => tsx(registers),
        // Instruction::PHP => self.php(bus),
        // Instruction::PLP => self.plp(&read),
        // Instruction::PHA => self.pha(&write),
        // Instruction::PLA => self.pla(&read),
        // Instruction::ADC => self.adc(&code, opeland, &read),
        // Instruction::SBC => self.sbc(&code, opeland, &read),
        // Instruction::CPX => self.cpx(&code, opeland, &read),
        // Instruction::CPY => self.cpy(&code, opeland, &read),
        // Instruction::CMP => self.cmp(&code, opeland, &read),
        // Instruction::AND => self.and(&code, opeland, &read),
        // Instruction::EOR => self.eor(&code, opeland, &read),
        // Instruction::ORA => self.ora(&code, opeland, &read),
        // Instruction::BIT => self.bit(opeland, &read),
        // Instruction::ASL => self.asl(&code, opeland, &read, &write),
        // Instruction::LSR => self.lsr(&code, opeland, &read, &write),
        // Instruction::ROL => self.rol(&code, opeland, &read, &write),
        // Instruction::ROR => self.ror(&code, opeland, &read, &write),
        // Instruction::INX => self.inx(),
        // Instruction::INY => self.iny(),
        // Instruction::INC => self.inc(opeland, &read, &write),
        // Instruction::DEX => self.dex(),
        // Instruction::DEY => self.dey(),
        // Instruction::DEC => self.dec(opeland, &read, &write),
        // Instruction::CLC => self.clc(),
        // Instruction::CLI => self.cli(),
        // Instruction::CLV => self.clv(),
        // Instruction::SEC => self.sec(),
        // Instruction::SEI => self.sei(),
        // Instruction::NOP => (),
        // Instruction::BRK => self.brk(&read, &write),
        // Instruction::JSR => self.jsr(opeland, &write),
        // Instruction::JMP => self.jmp(opeland),
        // Instruction::RTI => self.rti(&read),
        // Instruction::RTS => self.rts(&read),
        // Instruction::BPL => self.bpl(opeland),
        // Instruction::BMI => self.bmi(opeland),
        // Instruction::BVC => self.bvc(opeland),
        // Instruction::BVS => self.bvs(opeland),
        // Instruction::BCC => self.bcc(opeland),
        // Instruction::BCS => self.bcs(opeland),
        // Instruction::BNE => self.bne(opeland),
        // Instruction::BEQ => self.beq(opeland),
        // Instruction::SED => self.sed(),
        // Instruction::CLD => self.cld(),
        // Instruction::LAX => println!("{}", "TODO:Undocumented instruction"),
        // Instruction::SAX => println!("{}", "TODO:Undocumented instruction"),
        // Instruction::DCP => println!("{}", "TODO:Undocumented instruction"),
        // Instruction::ISB => println!("{}", "TODO:Undocumented instruction"),
        // Instruction::SLO => println!("{}", "TODO:Undocumented instruction"),
        // Instruction::RLA => println!("{}", "TODO:Undocumented instruction"),
        // Instruction::SRE => println!("{}", "TODO:Undocumented instruction"),
        // Instruction::RRA => println!("{}", "TODO:Undocumented instruction"),
        _ => println!("{}", "TODO:Undocumented instruction"),
    }
    code.cycle
}
