mod opecode;
mod fetch;
mod instructions;

use std::fmt::Debug;
use self::opecode::*;
use self::fetch::*;
use self::instructions::*;

use super::cpu_registers::CpuRegisters;
use super::bus::cpu_bus::CpuBus;
use super::types::Data;

pub fn reset<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    let pc = bus.read_word(0xFFFC);
    registers.set_PC(pc);
}

pub fn run<T: CpuRegisters + Debug, U: CpuBus>(registers: &mut T, bus: &mut U) -> Data {
    let _code = fetch(registers, bus);
    // println!("registers = {:?}", &registers);
    let ref map = opecode::MAP;
    let code = &*map.get(&_code).unwrap();
    let opeland = fetch_opeland(&code, registers, bus);
    // println!("opecode = {}, pc = {}, opeland = {}", &_code, &registers.get_PC(), opeland);
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
        Instruction::PHP => php(registers, bus),
        Instruction::PLP => plp(registers, bus),
        Instruction::PHA => pha(registers, bus),
        Instruction::PLA => pla(registers, bus),
        Instruction::ADC if code.mode == Addressing::Immediate => adc_imm(opeland, registers),
        Instruction::ADC => adc(opeland, registers, bus),
        Instruction::SBC if code.mode == Addressing::Immediate => sbc_imm(opeland, registers),
        Instruction::SBC => sbc(opeland, registers, bus),       
        Instruction::CPX if code.mode == Addressing::Immediate => cpx_imm(opeland, registers),
        Instruction::CPX => cpx(opeland, registers, bus),    
        Instruction::CPY if code.mode == Addressing::Immediate => cpy_imm(opeland, registers),
        Instruction::CPY => cpy(opeland, registers, bus),                    
        Instruction::CMP if code.mode == Addressing::Immediate => cmp_imm(opeland, registers),
        Instruction::CMP => cmp(opeland, registers, bus),      
        Instruction::AND if code.mode == Addressing::Immediate => and_imm(opeland, registers),
        Instruction::AND => and(opeland, registers, bus),  
        Instruction::EOR if code.mode == Addressing::Immediate => eor_imm(opeland, registers),
        Instruction::EOR => eor(opeland, registers, bus),  
        Instruction::ORA if code.mode == Addressing::Immediate => ora_imm(opeland, registers),
        Instruction::ORA => ora(opeland, registers, bus),  
        Instruction::BIT => bit(opeland, registers, bus),
        Instruction::ASL if code.mode == Addressing::Accumulator => asl_acc(registers),
        Instruction::ASL => asl(opeland, registers, bus),          
        Instruction::LSR if code.mode == Addressing::Accumulator => lsr_acc(registers),
        Instruction::LSR => lsr(opeland, registers, bus),  
        Instruction::ROL if code.mode == Addressing::Accumulator => rol_acc(registers),
        Instruction::ROL => rol(opeland, registers, bus),  
        Instruction::ROR if code.mode == Addressing::Accumulator => ror_acc(registers),
        Instruction::ROR => ror(opeland, registers, bus),  
        Instruction::INX => inx(registers),
        Instruction::INY => iny(registers),
        Instruction::INC => inc(opeland, registers, bus),
        Instruction::DEX => dex(registers),
        Instruction::DEY => dey(registers),
        Instruction::DEC => dec(opeland, registers, bus),
        Instruction::CLC => clc(registers),
        Instruction::CLI => cli(registers),
        Instruction::CLV => clv(registers),
        Instruction::SEC => sec(registers),
        Instruction::SEI => sei(registers),
        Instruction::NOP => (),
        Instruction::BRK => brk(registers, bus),
        Instruction::JSR => jsr(opeland, registers, bus),
        Instruction::JMP => jmp(opeland, registers),
        Instruction::RTI => rti(registers, bus),
        Instruction::RTS => rts(registers, bus),
        Instruction::BCC => bcc(opeland, registers),
        Instruction::BPL => bpl(opeland, registers),
        Instruction::BMI => bmi(opeland, registers),
        Instruction::BVC => bvc(opeland, registers),
        Instruction::BVS => bvs(opeland, registers),
        Instruction::BCS => bcs(opeland, registers),
        Instruction::BNE => bne(opeland, registers),
        Instruction::BEQ => beq(opeland, registers),
        Instruction::SED => sed(registers),
        Instruction::CLD => cld(registers),
        Instruction::LAX => println!("{}", "TODO:Undocumented instruction"),
        Instruction::SAX => println!("{}", "TODO:Undocumented instruction"),
        Instruction::DCP => println!("{}", "TODO:Undocumented instruction"),
        Instruction::ISB => println!("{}", "TODO:Undocumented instruction"),
        Instruction::SLO => println!("{}", "TODO:Undocumented instruction"),
        Instruction::RLA => println!("{}", "TODO:Undocumented instruction"),
        Instruction::SRE => println!("{}", "TODO:Undocumented instruction"),
        Instruction::RRA => println!("{}", "TODO:Undocumented instruction"),
        _ => panic!("{}", "Undefined opecode detected."),
    }
    code.cycle
}
