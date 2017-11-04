use super::super::cpu_registers::CpuRegisters;
use super::super::bus::cpu_bus::CpuBus;
use super::super::types::{Data, Addr, Word};

pub fn lda<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let computed = bus.read(opeland);
    registers
        .set_A(computed)
        .update_negative_by(computed)
        .update_zero_by(computed);
}

pub fn lda_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    registers
        .set_A(opeland as Data)
        .update_negative_by(opeland as Data)
        .update_zero_by(opeland as Data);
}

pub fn ldx<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let computed = bus.read(opeland);
    registers
        .set_X(computed)
        .update_negative_by(computed)
        .update_zero_by(computed);
}

pub fn ldx_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    registers
        .set_X(opeland as Data)
        .update_negative_by(opeland as Data)
        .update_zero_by(opeland as Data);
}

pub fn ldy<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let computed = bus.read(opeland);
    registers
        .set_Y(computed)
        .update_negative_by(computed)
        .update_zero_by(computed);
}

pub fn ldy_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    registers
        .set_Y(opeland as Data)
        .update_negative_by(opeland as Data)
        .update_zero_by(opeland as Data);
}

pub fn sta<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    bus.write(opeland, registers.get_A());
}

pub fn stx<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    bus.write(opeland, registers.get_X());
}

pub fn sty<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    bus.write(opeland, registers.get_Y());
}

pub fn txa<T: CpuRegisters>(registers: &mut T) {
    let x = registers.get_X();
    registers
        .set_A(x)
        .update_negative_by(x)
        .update_zero_by(x);
}

pub fn tya<T: CpuRegisters>(registers: &mut T) {
    let y = registers.get_Y();
    registers
        .set_A(y)
        .update_negative_by(y)
        .update_zero_by(y);
}

pub fn txs<T: CpuRegisters>(registers: &mut T) {
    let x = registers.get_X();
    registers.set_SP(x);
}

pub fn tay<T: CpuRegisters>(registers: &mut T) {
    let acc = registers.get_A();
    registers
        .set_Y(acc)
        .update_negative_by(acc)
        .update_zero_by(acc);
}

pub fn tax<T: CpuRegisters>(registers: &mut T) {
    let acc = registers.get_A();
    registers
        .set_X(acc)
        .update_negative_by(acc)
        .update_zero_by(acc);
}

pub fn tsx<T: CpuRegisters>(registers: &mut T) {
    let sp = registers.get_SP();
    registers
        .set_X(sp)
        .update_negative_by(sp)
        .update_zero_by(sp);
}

pub fn php<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    registers.set_break(true);
    push_status(registers, bus);
}

pub fn plp<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    registers.set_reserved(true);
    let status = pop(registers, bus);
    registers.set_P(status);
}

pub fn pha<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    let acc = registers.get_A();
    push(acc, registers, bus);
}

fn push<T: CpuRegisters, U: CpuBus>(data: Data, registers: &mut T, bus: &mut U) {
    let addr = registers.get_SP() as Addr;
    bus.write((addr | 0x0100), data);
    registers.dec_SP();
}

fn push_status<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    let status = registers.get_P();
    push(status, registers, bus);
}

fn pop<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Data {
    registers.inc_SP();
    let addr = 0x0100 | registers.get_SP() as Addr;
    bus.read(addr)
}


/*
    fn branch(&self, addr: Addr) {
        self.registers.borrow_mut().set_pc(addr);
    }

    fn push_pc<W>(&self, write: W)
        where W: Fn(Addr, Data)
    {
        let pc = self.registers.borrow().get_pc();
        self.push((pc >> 8) as u8, &write);
        self.push(pc as u8, &write);
    }


    fn pop_pc<R>(&self, read: R)
        where R: Fn(Addr) -> Data
    {
        let lower = self.pop(&read) as u16;
        let upper = self.pop(&read) as u16;
        self.registers.borrow_mut().set_pc(upper << 8 | lower);
    }

    fn pop_status<R>(&self, read: R)
        where R: Fn(Addr) -> Data
    {
        let status = self.pop(&read) as u8;
        self.registers.borrow_mut().set_p(status);
    }

   fn pla<R>(&self, ref read: R)
        where R: Fn(Addr) -> Data
    {
        let v = self.pop(&read);
        self.registers
            .borrow_mut()
            .set_acc(v)
            .update_negative(v)
            .update_zero(v);
    }

    fn adc<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = fetched + self.registers.borrow().get(ByteRegister::A) +
                       bool_to_u8(self.registers.borrow().get_status(StatusName::carry));
        self.registers
            .borrow_mut()
            .update_overflow(fetched, computed)
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed > 0xFF as u8)
            .set_acc(computed);
    }

    fn sbc<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::A) - fetched -
                       bool_to_u8(!self.registers.borrow().get_status(StatusName::carry));
        self.registers
            .borrow_mut()
            .update_overflow(computed, fetched)
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed >= 0 as u8)
            .set_acc(computed);
    }

    fn cpx<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::X) - fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed >= 0 as u8);
    }

    fn cpy<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::Y) - fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed >= 0 as u8);
    }

    fn cmp<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::A) - fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed >= 0 as u8);
    }

    fn and<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::A) & fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_acc(computed);
    }

    fn eor<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::A) ^ fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_acc(computed);
    }

    fn ora<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::A) | fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_acc(computed);
    }

    fn bit<R>(&self, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = read(opeland);
        self.registers
            .borrow_mut()
            .update_negative(fetched)
            .update_zero(fetched & self.registers.borrow().get(ByteRegister::A))
            .set_overflow(fetched & 0x40 == 0x40);
    }

    fn asl<R, W>(&self, code: &Opecode, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        match code.mode {
            Addressing::Accumulator => {
                let acc = self.registers.borrow().get(ByteRegister::A);
                let shifted = (acc << 1) as u8;
                self.registers
                    .borrow_mut()
                    .set_carry(acc & 0x80 == 0x80)
                    .update_negative(shifted)
                    .update_zero(shifted)
                    .set_acc(shifted);
            }
            _ => {
                let fetched = read(opeland);
                let shifted = (fetched << 1) as u8;
                self.registers
                    .borrow_mut()
                    .set_carry(fetched & 0x80 == 0x80)
                    .update_negative(shifted)
                    .update_zero(shifted);
                write(opeland, shifted);
            }
        };
    }

    fn lsr<R, W>(&self, code: &Opecode, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        match code.mode {
            Addressing::Accumulator => {
                let acc = self.registers.borrow().get(ByteRegister::A);
                let shifted = (acc >> 1) as u8;
                self.registers
                    .borrow_mut()
                    .set_carry(acc & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted)
                    .set_acc(shifted);
            }
            _ => {
                let fetched = read(opeland);
                let shifted = (fetched >> 1) as u8;
                self.registers
                    .borrow_mut()
                    .set_carry(fetched & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted);
                write(opeland, shifted);
            }
        };
    }

    fn ror<R, W>(&self, code: &Opecode, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        let shift = |v: u8| {
            ((v >> 1) |
             if self.registers.borrow().get_status(StatusName::carry) {
                 0x80
             } else {
                 0x00
             }) as u8
        };
        match code.mode {
            Addressing::Accumulator => {
                let acc = self.registers.borrow().get(ByteRegister::A);
                let shifted = shift(acc);
                self.registers
                    .borrow_mut()
                    .set_carry(acc & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted)
                    .set_acc(shifted);
            }
            _ => {
                let fetched = read(opeland);
                let shifted = shift(fetched);
                self.registers
                    .borrow_mut()
                    .set_carry(fetched & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted);
                write(opeland, shifted);
            }
        };
    }

    fn rol<R, W>(&self, code: &Opecode, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        let shift = |v: u8| {
            ((v << 1) |
             if self.registers.borrow().get_status(StatusName::carry) {
                 0x01
             } else {
                 0x00
             }) as u8
        };
        match code.mode {
            Addressing::Accumulator => {
                let acc = self.registers.borrow().get(ByteRegister::A);
                let shifted = shift(acc);
                self.registers
                    .borrow_mut()
                    .set_carry(acc & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted)
                    .set_acc(shifted);
            }
            _ => {
                let fetched = read(opeland);
                let shifted = shift(fetched);
                self.registers
                    .borrow_mut()
                    .set_carry(fetched & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted);
                write(opeland, shifted);
            }
        };
    }

    fn inx(&self) {
        let x = self.registers.borrow().get(ByteRegister::X) + 1;
        self.registers
            .borrow_mut()
            .set_x(x)
            .update_negative(x)
            .update_zero(x);
    }

    fn iny(&self) {
        let y = self.registers.borrow().get(ByteRegister::Y) + 1;
        self.registers
            .borrow_mut()
            .set_y(y)
            .update_negative(y)
            .update_zero(y);
    }

    fn inc<R, W>(&self, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        let data = read(opeland) + 1 as u8;
        self.registers
            .borrow_mut()
            .update_negative(data)
            .update_zero(data);
        write(opeland, data);
    }

    fn dex(&self) {
        let x = self.registers.borrow().get(ByteRegister::X) - 1;
        self.registers
            .borrow_mut()
            .set_x(x)
            .update_negative(x)
            .update_zero(x);
    }

    fn dey(&self) {
        let y = self.registers.borrow().get(ByteRegister::Y) - 1;
        self.registers
            .borrow_mut()
            .set_y(y)
            .update_negative(y)
            .update_zero(y);
    }

    fn dec<R, W>(&self, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        let data = read(opeland) - 1 as u8;
        self.registers
            .borrow_mut()
            .update_negative(data)
            .update_zero(data);
        write(opeland, data);
    }

    fn clc(&self) {
        self.registers.borrow_mut().set_carry(false);
    }

    fn cli(&self) {
        self.registers.borrow_mut().set_interrupt(false);
    }

    fn clv(&self) {
        self.registers.borrow_mut().set_overflow(false);
    }

    fn sec(&self) {
        self.registers.borrow_mut().set_carry(true);
    }

    fn sei(&self) {
        self.registers.borrow_mut().set_interrupt(true);
    }

    fn brk<R, W>(&self, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        let interrupt = self.registers
            .borrow()
            .get_status(StatusName::interrupt);
        self.registers.borrow_mut().inc_pc();
        self.push_pc(&write);
        self.registers.borrow_mut().set_break(true);
        self.push_status(&write);
        self.registers.borrow_mut().set_interrupt(true);
        // Ignore interrupt when already set.
        if !interrupt {
            let fetched = self.read_word(&read, 0xFFFE);
            self.registers.borrow_mut().set_pc(fetched);
        }
        self.registers.borrow_mut().dec_pc();
    }

    fn jsr<W>(&self, opeland: Word, write: W)
        where W: Fn(Addr, Data)
    {
        let pc = self.registers.borrow().get_pc() - 1;
        self.push((pc >> 8) as u8, &write);
        self.push(pc as u8, &write);
        self.registers.borrow_mut().set_pc(opeland);
    }

    fn jmp(&self, opeland: Word) {
        self.registers.borrow_mut().set_pc(opeland);
    }

    fn rti<R>(&self, read: R)
        where R: Fn(Addr) -> Data
    {
        self.pop_status(&read);
        self.pop_pc(&read);
        self.registers.borrow_mut().set_reserved(true);
    }

    fn rts<R>(&self, read: R)
        where R: Fn(Addr) -> Data
    {
        self.pop_pc(&read);
        self.registers.borrow_mut().inc_pc();
    }

    fn bcc(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::carry) {
            self.branch(opeland)
        }
    }

    fn bcs(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::carry) {
            self.branch(opeland)
        }
    }

    fn beq(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::zero) {
            self.branch(opeland)
        }
    }

    fn bmi(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::negative) {
            self.branch(opeland)
        }
    }

    fn bne(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::zero) {
            self.branch(opeland)
        }
    }

    fn bpl(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::negative) {
            self.branch(opeland)
        }
    }

    fn bvs(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::overflow) {
            self.branch(opeland)
        }
    }

    fn bvc(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::overflow) {
            self.branch(opeland)
        }
    }

    fn cld(&self) {
        self.registers.borrow_mut().set_decimal(true);
    }

    fn sed(&self) {
        self.registers.borrow_mut().set_decimal(true);
    }
    */


#[cfg(test)]
mod test {
    use super::super::super::cpu_registers::Registers;
    use super::*;

    struct MockBus {
        pub mem: Vec<Data>,
    }

    impl MockBus {
        pub fn new() -> Self {
            MockBus { mem: vec!(0; 1024) }
        }
    }

    impl CpuBus for MockBus {
        fn read(&self, addr: Addr) -> Data {
            self.mem[addr as usize]
        }
        fn read_word(&self, addr: Addr) -> Word {
            let lower = self.read(addr) as u16;
            let upper = self.read(addr + 1) as u16;
            (upper << 8 | lower) as u16
        }
        fn write(&mut self, addr: Addr, data: Data) {
            self.mem[addr as usize] = data;
        }
    }

    #[test]
    fn test_lda_immidiate() {
        let mut reg = Registers::new();
        lda_imm(0xA5, &mut reg);
        assert_eq!(reg.get_A(), 0xA5);
    }

    #[test]
    fn test_lda() {
        let mut reg = Registers::new();
        let mut bus = MockBus::new();
        bus.mem[0xAA] = 0xA5;
        lda(0xAA, &mut reg, &mut bus);
        assert_eq!(reg.get_A(), 0xA5);
    }

    #[test]
    fn test_ldx_immidiate() {
        let mut reg = Registers::new();
        ldx_imm(0xA5, &mut reg);
        assert_eq!(reg.get_X(), 0xA5);
    }

    #[test]
    fn test_ldx() {
        let mut reg = Registers::new();
        let mut bus = MockBus::new();
        bus.mem[0xAA] = 0xA5;
        ldx(0xAA, &mut reg, &mut bus);
        assert_eq!(reg.get_X(), 0xA5);
    }

    #[test]
    fn test_ldy_immidiate() {
        let mut reg = Registers::new();
        ldy_imm(0xA5, &mut reg);
        assert_eq!(reg.get_Y(), 0xA5);
    }

    #[test]
    fn test_ldy() {
        let mut reg = Registers::new();
        let mut bus = MockBus::new();
        bus.mem[0xAA] = 0xA5;
        ldy(0xAA, &mut reg, &mut bus);
        assert_eq!(reg.get_Y(), 0xA5);
    }

    #[test]
    fn test_sta() {
        let mut reg = Registers::new();
        reg.set_A(0xA5);
        let mut bus = MockBus::new();
        sta(0xAA, &mut reg, &mut bus);
        assert_eq!(bus.mem[0xAA], 0xA5);
    }

    #[test]
    fn test_stx() {
        let mut reg = Registers::new();
        reg.set_X(0xA5);
        let mut bus = MockBus::new();
        stx(0xAA, &mut reg, &mut bus);
        assert_eq!(bus.mem[0xAA], 0xA5);
    }

    #[test]
    fn test_sty() {
        let mut reg = Registers::new();
        reg.set_Y(0xA5);
        let mut bus = MockBus::new();
        sty(0xAA, &mut reg, &mut bus);
        assert_eq!(bus.mem[0xAA], 0xA5);
    }

    #[test]
    fn test_txa() {
        let mut reg = Registers::new();
        reg.set_X(0xA5);
        txa(&mut reg);
        assert_eq!(reg.get_A(), 0xA5);
    }

    #[test]
    fn test_tax() {
        let mut reg = Registers::new();
        reg.set_A(0xA5);
        tax(&mut reg);
        assert_eq!(reg.get_X(), 0xA5);
    }

    #[test]
    fn test_tay() {
        let mut reg = Registers::new();
        reg.set_A(0xA5);
        tay(&mut reg);
        assert_eq!(reg.get_Y(), 0xA5);
    }


    #[test]
    fn test_tya() {
        let mut reg = Registers::new();
        reg.set_Y(0xA5);
        tya(&mut reg);
        assert_eq!(reg.get_A(), 0xA5);
    }

    #[test]
    fn test_txs() {
        let mut reg = Registers::new();
        reg.set_X(0xA5);
        txs(&mut reg);
        assert_eq!(reg.get_SP(), 0xA5);
    }

    #[test]
    fn test_tsx() {
        let mut reg = Registers::new();
        reg.set_SP(0xA5);
        tsx(&mut reg);
        assert_eq!(reg.get_X(), 0xA5);
    }

    #[test]
    fn test_php() {
        let mut reg = Registers::new();
        reg.set_SP(0xA5);
        let mut bus = MockBus::new();
        php(&mut reg, &mut bus);
        assert_eq!(bus.mem[0x01A5], 0x34);
    }

    #[test]
    fn test_plp() {
        let mut reg = Registers::new();
        reg.set_SP(0xA5);
        let mut bus = MockBus::new();
        bus.mem[0x1A6] = 0xA5;
        plp(&mut reg, &mut bus);
        assert_eq!(reg.get_P(), 0xA5);
    }

    #[test]
    fn test_pha() {
        let mut reg = Registers::new();
        reg.set_SP(0xA5).set_A(0x5A);
        let mut bus = MockBus::new();
        pha(&mut reg, &mut bus);
        assert_eq!(bus.mem[0x01A5], 0x5A);
    }

}

/*





#[test]
fn adc_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x05);
    let code = Opecode {
        name: Instruction::ADC,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.adc(&code, 0xA5, |addr: Addr| 0 /* mock */);
    assert!(cpu.registers.borrow().get(ByteRegister::A) == 0xAA);
}

#[test]
fn sbc_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x10);
    let code = Opecode {
        name: Instruction::SBC,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.sbc(&code, 0x06, |addr: Addr| 0 /* mock */);
    assert!(cpu.registers.borrow().get(ByteRegister::A) == 0x09);
}

#[test]
fn cpx_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_x(0x05);
    let code = Opecode {
        name: Instruction::CPX,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.cpx(&code, 0x04, |addr: Addr| 0 /* mock */);
    assert!(cpu.registers.borrow().get_status(StatusName::carry));
}

#[test]
fn cpy_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_y(0x05);
    let code = Opecode {
        name: Instruction::CPY,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.cpy(&code, 0x04, |addr: Addr| 0 /* mock */);
    assert!(cpu.registers.borrow().get_status(StatusName::carry));
}

#[test]
fn cmp_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x05);
    let code = Opecode {
        name: Instruction::CMP,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.cmp(&code, 0x04, |addr: Addr| 0 /* mock */);
    assert!(cpu.registers.borrow().get_status(StatusName::carry));
}

#[test]
fn and_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0xA5);
    let code = Opecode {
        name: Instruction::AND,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.and(&code, 0x05, |addr: Addr| 0 /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0x05);
}

#[test]
fn eor_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0xA5);
    let code = Opecode {
        name: Instruction::EOR,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.eor(&code, 0x05, |addr: Addr| 0 /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0xA0);
}

#[test]
fn ora_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0xA0);
    let code = Opecode {
        name: Instruction::ORA,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.ora(&code, 0x05, |addr: Addr| 0 /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0xA5);
}

#[test]
fn asl_accumlator() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x55);
    let code = Opecode {
        name: Instruction::ASL,
        mode: Addressing::Accumulator,
        cycle: 1, // mock
    };
    cpu.asl(&code,
            0x00,
            |addr: Addr| 0,
            |addr: Addr, data: Data| () /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0xAA);
}

#[test]
fn lsr_accumlator() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x55);
    let code = Opecode {
        name: Instruction::LSR,
        mode: Addressing::Accumulator,
        cycle: 1, // mock
    };
    cpu.lsr(&code,
            0x00,
            |addr: Addr| 0,
            |addr: Addr, data: Data| () /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0x2A);
}

#[test]
fn ror_accumlator_with_carry() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x55);
    cpu.registers.borrow_mut().set_carry(true);
    let code = Opecode {
        name: Instruction::ROR,
        mode: Addressing::Accumulator,
        cycle: 1, // mock
    };
    cpu.ror(&code,
            0x00,
            |addr: Addr| 0,
            |addr: Addr, data: Data| () /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0xAA);
}

#[test]
fn ror_accumlator_without_carry() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000).set_acc(0x55);
    let code = Opecode {
        name: Instruction::ROR,
        mode: Addressing::Accumulator,
        cycle: 1, // mock
    };
    cpu.ror(&code,
            0x00,
            |addr: Addr| 0,
            |addr: Addr, data: Data| () /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0x2A);
}


#[test]
fn rol_accumlator_with_carry() {
    let mut cpu = Cpu::new();
    cpu.registers
        .borrow_mut()
        .set_pc(0x0000)
        .set_acc(0x55)
        .set_carry(true);
    let code = Opecode {
        name: Instruction::ROL,
        mode: Addressing::Accumulator,
        cycle: 1, // mock
    };
    cpu.rol(&code,
            0x00,
            |addr: Addr| 0,
            |addr: Addr, data: Data| () /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0xAB);
}

#[test]
fn inx() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000).set_x(0xA5);
    let code = Opecode {
        name: Instruction::INX,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.inx();
    assert_eq!(cpu.registers.borrow().get(ByteRegister::X), 0xA6);
}

#[test]
fn iny() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000).set_y(0xA5);
    let code = Opecode {
        name: Instruction::INY,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.iny();
    assert_eq!(cpu.registers.borrow().get(ByteRegister::Y), 0xA6);
}

#[test]
fn inc() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    let code = Opecode {
        name: Instruction::INC,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.inc(0x55, |addr: Addr| 0xA5, |addr: Addr, data: Data| {
        assert_eq!(addr, 0x55);
        assert_eq!(data, 0xA6);
    });
}

#[test]
fn dex() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000).set_x(0xA5);
    let code = Opecode {
        name: Instruction::DEX,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.dex();
    assert_eq!(cpu.registers.borrow().get(ByteRegister::X), 0xA4);
}

#[test]
fn dey() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000).set_y(0xA5);
    let code = Opecode {
        name: Instruction::DEY,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.dey();
    assert_eq!(cpu.registers.borrow().get(ByteRegister::Y), 0xA4);
}

#[test]
fn dec() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    let code = Opecode {
        name: Instruction::DEC,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.dec(0x55, |addr: Addr| 0xA5, |addr: Addr, data: Data| {
        assert_eq!(addr, 0x55);
        assert_eq!(data, 0xA4);
    });
}

*/