use crate::bus::Bus;

use super::{AddrFn, AddressMode, Cpu, Instruction, OpFn, StatusFlags};

impl Cpu {
    pub(super) fn build_lookup() -> Vec<Instruction> {
        fn ins(
            name: &'static str,
            operate: OpFn,
            addrmode_fn: AddrFn,
            addrmode: AddressMode,
            cycles: u8,
        ) -> Instruction {
            Instruction {
                name,
                operate,
                addrmode_fn,
                addrmode,
                cycles,
            }
        }

        let mut table = vec![ins("XXX", Cpu::xxx, Cpu::imp, AddressMode::Imp, 2); 256];

        macro_rules! set {
            ($op:expr, $name:expr, $operate:ident, $addrmode:ident, $mode:expr, $cycles:expr) => {
                table[$op] = ins($name, Cpu::$operate, Cpu::$addrmode, $mode, $cycles);
            };
        }

        set!(0x00, "BRK", brk, imm, AddressMode::Imm, 7);
        set!(0x01, "ORA", ora, izx, AddressMode::Izx, 6);
        set!(0x04, "NOP", nop, imp, AddressMode::Imp, 3);
        set!(0x05, "ORA", ora, zp0, AddressMode::Zp0, 3);
        set!(0x06, "ASL", asl, zp0, AddressMode::Zp0, 5);
        set!(0x08, "PHP", php, imp, AddressMode::Imp, 3);
        set!(0x09, "ORA", ora, imm, AddressMode::Imm, 2);
        set!(0x0A, "ASL", asl, imp, AddressMode::Imp, 2);
        set!(0x0C, "NOP", nop, imp, AddressMode::Imp, 4);
        set!(0x0D, "ORA", ora, abs, AddressMode::Abs, 4);
        set!(0x0E, "ASL", asl, abs, AddressMode::Abs, 6);

        set!(0x10, "BPL", bpl, rel, AddressMode::Rel, 2);
        set!(0x11, "ORA", ora, izy, AddressMode::Izy, 5);
        set!(0x14, "NOP", nop, imp, AddressMode::Imp, 4);
        set!(0x15, "ORA", ora, zpx, AddressMode::Zpx, 4);
        set!(0x16, "ASL", asl, zpx, AddressMode::Zpx, 6);
        set!(0x18, "CLC", clc, imp, AddressMode::Imp, 2);
        set!(0x19, "ORA", ora, aby, AddressMode::Aby, 4);
        set!(0x1A, "NOP", nop, imp, AddressMode::Imp, 2);
        set!(0x1C, "NOP", nop, imp, AddressMode::Imp, 7);
        set!(0x1D, "ORA", ora, abx, AddressMode::Abx, 4);
        set!(0x1E, "ASL", asl, abx, AddressMode::Abx, 7);

        set!(0x20, "JSR", jsr, abs, AddressMode::Abs, 6);
        set!(0x21, "AND", and, izx, AddressMode::Izx, 6);
        set!(0x24, "BIT", bit, zp0, AddressMode::Zp0, 3);
        set!(0x25, "AND", and, zp0, AddressMode::Zp0, 3);
        set!(0x26, "ROL", rol, zp0, AddressMode::Zp0, 5);
        set!(0x28, "PLP", plp, imp, AddressMode::Imp, 4);
        set!(0x29, "AND", and, imm, AddressMode::Imm, 2);
        set!(0x2A, "ROL", rol, imp, AddressMode::Imp, 2);
        set!(0x2C, "BIT", bit, abs, AddressMode::Abs, 4);
        set!(0x2D, "AND", and, abs, AddressMode::Abs, 4);
        set!(0x2E, "ROL", rol, abs, AddressMode::Abs, 6);

        set!(0x30, "BMI", bmi, rel, AddressMode::Rel, 2);
        set!(0x31, "AND", and, izy, AddressMode::Izy, 5);
        set!(0x34, "NOP", nop, imp, AddressMode::Imp, 4);
        set!(0x35, "AND", and, zpx, AddressMode::Zpx, 4);
        set!(0x36, "ROL", rol, zpx, AddressMode::Zpx, 6);
        set!(0x38, "SEC", sec, imp, AddressMode::Imp, 2);
        set!(0x39, "AND", and, aby, AddressMode::Aby, 4);
        set!(0x3A, "NOP", nop, imp, AddressMode::Imp, 2);
        set!(0x3C, "NOP", nop, imp, AddressMode::Imp, 7);
        set!(0x3D, "AND", and, abx, AddressMode::Abx, 4);
        set!(0x3E, "ROL", rol, abx, AddressMode::Abx, 7);

        set!(0x40, "RTI", rti, imp, AddressMode::Imp, 6);
        set!(0x41, "EOR", eor, izx, AddressMode::Izx, 6);
        set!(0x45, "EOR", eor, zp0, AddressMode::Zp0, 3);
        set!(0x46, "LSR", lsr, zp0, AddressMode::Zp0, 5);
        set!(0x48, "PHA", pha, imp, AddressMode::Imp, 3);
        set!(0x49, "EOR", eor, imm, AddressMode::Imm, 2);
        set!(0x4A, "LSR", lsr, imp, AddressMode::Imp, 2);
        set!(0x4C, "JMP", jmp, abs, AddressMode::Abs, 3);
        set!(0x4D, "EOR", eor, abs, AddressMode::Abs, 4);
        set!(0x4E, "LSR", lsr, abs, AddressMode::Abs, 6);

        set!(0x50, "BVC", bvc, rel, AddressMode::Rel, 2);
        set!(0x51, "EOR", eor, izy, AddressMode::Izy, 5);
        set!(0x55, "EOR", eor, zpx, AddressMode::Zpx, 4);
        set!(0x56, "LSR", lsr, zpx, AddressMode::Zpx, 6);
        set!(0x58, "CLI", cli, imp, AddressMode::Imp, 2);
        set!(0x59, "EOR", eor, aby, AddressMode::Aby, 4);
        set!(0x5A, "NOP", nop, imp, AddressMode::Imp, 2);
        set!(0x5C, "NOP", nop, imp, AddressMode::Imp, 7);
        set!(0x5D, "EOR", eor, abx, AddressMode::Abx, 4);
        set!(0x5E, "LSR", lsr, abx, AddressMode::Abx, 7);

        set!(0x60, "RTS", rts, imp, AddressMode::Imp, 6);
        set!(0x61, "ADC", adc, izx, AddressMode::Izx, 6);
        set!(0x65, "ADC", adc, zp0, AddressMode::Zp0, 3);
        set!(0x66, "ROR", ror, zp0, AddressMode::Zp0, 5);
        set!(0x68, "PLA", pla, imp, AddressMode::Imp, 4);
        set!(0x69, "ADC", adc, imm, AddressMode::Imm, 2);
        set!(0x6A, "ROR", ror, imp, AddressMode::Imp, 2);
        set!(0x6C, "JMP", jmp, ind, AddressMode::Ind, 5);
        set!(0x6D, "ADC", adc, abs, AddressMode::Abs, 4);
        set!(0x6E, "ROR", ror, abs, AddressMode::Abs, 6);

        set!(0x70, "BVS", bvs, rel, AddressMode::Rel, 2);
        set!(0x71, "ADC", adc, izy, AddressMode::Izy, 5);
        set!(0x75, "ADC", adc, zpx, AddressMode::Zpx, 4);
        set!(0x76, "ROR", ror, zpx, AddressMode::Zpx, 6);
        set!(0x78, "SEI", sei, imp, AddressMode::Imp, 2);
        set!(0x79, "ADC", adc, aby, AddressMode::Aby, 4);
        set!(0x7A, "NOP", nop, imp, AddressMode::Imp, 2);
        set!(0x7C, "NOP", nop, imp, AddressMode::Imp, 7);
        set!(0x7D, "ADC", adc, abx, AddressMode::Abx, 4);
        set!(0x7E, "ROR", ror, abx, AddressMode::Abx, 7);

        set!(0x80, "NOP", nop, imp, AddressMode::Imp, 2);
        set!(0x81, "STA", sta, izx, AddressMode::Izx, 6);
        set!(0x84, "STY", sty, zp0, AddressMode::Zp0, 3);
        set!(0x85, "STA", sta, zp0, AddressMode::Zp0, 3);
        set!(0x86, "STX", stx, zp0, AddressMode::Zp0, 3);
        set!(0x88, "DEY", dey, imp, AddressMode::Imp, 2);
        set!(0x8A, "TXA", txa, imp, AddressMode::Imp, 2);
        set!(0x8C, "STY", sty, abs, AddressMode::Abs, 4);
        set!(0x8D, "STA", sta, abs, AddressMode::Abs, 4);
        set!(0x8E, "STX", stx, abs, AddressMode::Abs, 4);

        set!(0x90, "BCC", bcc, rel, AddressMode::Rel, 2);
        set!(0x91, "STA", sta, izy, AddressMode::Izy, 6);
        set!(0x94, "STY", sty, zpx, AddressMode::Zpx, 4);
        set!(0x95, "STA", sta, zpx, AddressMode::Zpx, 4);
        set!(0x96, "STX", stx, zpy, AddressMode::Zpy, 4);
        set!(0x98, "TYA", tya, imp, AddressMode::Imp, 2);
        set!(0x99, "STA", sta, aby, AddressMode::Aby, 5);
        set!(0x9A, "TXS", txs, imp, AddressMode::Imp, 2);
        set!(0x9C, "NOP", nop, imp, AddressMode::Imp, 5);
        set!(0x9D, "STA", sta, abx, AddressMode::Abx, 5);

        set!(0xA0, "LDY", ldy, imm, AddressMode::Imm, 2);
        set!(0xA1, "LDA", lda, izx, AddressMode::Izx, 6);
        set!(0xA2, "LDX", ldx, imm, AddressMode::Imm, 2);
        set!(0xA4, "LDY", ldy, zp0, AddressMode::Zp0, 3);
        set!(0xA5, "LDA", lda, zp0, AddressMode::Zp0, 3);
        set!(0xA6, "LDX", ldx, zp0, AddressMode::Zp0, 3);
        set!(0xA8, "TAY", tay, imp, AddressMode::Imp, 2);
        set!(0xA9, "LDA", lda, imm, AddressMode::Imm, 2);
        set!(0xAA, "TAX", tax, imp, AddressMode::Imp, 2);
        set!(0xAC, "LDY", ldy, abs, AddressMode::Abs, 4);
        set!(0xAD, "LDA", lda, abs, AddressMode::Abs, 4);
        set!(0xAE, "LDX", ldx, abs, AddressMode::Abs, 4);

        set!(0xB0, "BCS", bcs, rel, AddressMode::Rel, 2);
        set!(0xB1, "LDA", lda, izy, AddressMode::Izy, 5);
        set!(0xB4, "LDY", ldy, zpx, AddressMode::Zpx, 4);
        set!(0xB5, "LDA", lda, zpx, AddressMode::Zpx, 4);
        set!(0xB6, "LDX", ldx, zpy, AddressMode::Zpy, 4);
        set!(0xB8, "CLV", clv, imp, AddressMode::Imp, 2);
        set!(0xB9, "LDA", lda, aby, AddressMode::Aby, 4);
        set!(0xBA, "TSX", tsx, imp, AddressMode::Imp, 2);
        set!(0xBC, "LDY", ldy, abx, AddressMode::Abx, 4);
        set!(0xBD, "LDA", lda, abx, AddressMode::Abx, 4);
        set!(0xBE, "LDX", ldx, aby, AddressMode::Aby, 4);

        set!(0xC0, "CPY", cpy, imm, AddressMode::Imm, 2);
        set!(0xC1, "CMP", cmp, izx, AddressMode::Izx, 6);
        set!(0xC4, "CPY", cpy, zp0, AddressMode::Zp0, 3);
        set!(0xC5, "CMP", cmp, zp0, AddressMode::Zp0, 3);
        set!(0xC6, "DEC", dec, zp0, AddressMode::Zp0, 5);
        set!(0xC8, "INY", iny, imp, AddressMode::Imp, 2);
        set!(0xC9, "CMP", cmp, imm, AddressMode::Imm, 2);
        set!(0xCA, "DEX", dex, imp, AddressMode::Imp, 2);
        set!(0xCC, "CPY", cpy, abs, AddressMode::Abs, 4);
        set!(0xCD, "CMP", cmp, abs, AddressMode::Abs, 4);
        set!(0xCE, "DEC", dec, abs, AddressMode::Abs, 6);

        set!(0xD0, "BNE", bne, rel, AddressMode::Rel, 2);
        set!(0xD1, "CMP", cmp, izy, AddressMode::Izy, 5);
        set!(0xD5, "CMP", cmp, zpx, AddressMode::Zpx, 4);
        set!(0xD6, "DEC", dec, zpx, AddressMode::Zpx, 6);
        set!(0xD8, "CLD", cld, imp, AddressMode::Imp, 2);
        set!(0xD9, "CMP", cmp, aby, AddressMode::Aby, 4);
        set!(0xDA, "NOP", nop, imp, AddressMode::Imp, 2);
        set!(0xDC, "NOP", nop, imp, AddressMode::Imp, 7);
        set!(0xDD, "CMP", cmp, abx, AddressMode::Abx, 4);
        set!(0xDE, "DEC", dec, abx, AddressMode::Abx, 7);

        set!(0xE0, "CPX", cpx, imm, AddressMode::Imm, 2);
        set!(0xE1, "SBC", sbc, izx, AddressMode::Izx, 6);
        set!(0xE4, "CPX", cpx, zp0, AddressMode::Zp0, 3);
        set!(0xE5, "SBC", sbc, zp0, AddressMode::Zp0, 3);
        set!(0xE6, "INC", inc, zp0, AddressMode::Zp0, 5);
        set!(0xE8, "INX", inx, imp, AddressMode::Imp, 2);
        set!(0xE9, "SBC", sbc, imm, AddressMode::Imm, 2);
        set!(0xEA, "NOP", nop, imp, AddressMode::Imp, 2);
        set!(0xEC, "CPX", cpx, abs, AddressMode::Abs, 4);
        set!(0xED, "SBC", sbc, abs, AddressMode::Abs, 4);
        set!(0xEE, "INC", inc, abs, AddressMode::Abs, 6);

        set!(0xF0, "BEQ", beq, rel, AddressMode::Rel, 2);
        set!(0xF1, "SBC", sbc, izy, AddressMode::Izy, 5);
        set!(0xF5, "SBC", sbc, zpx, AddressMode::Zpx, 4);
        set!(0xF6, "INC", inc, zpx, AddressMode::Zpx, 6);
        set!(0xF8, "SED", sed, imp, AddressMode::Imp, 2);
        set!(0xF9, "SBC", sbc, aby, AddressMode::Aby, 4);
        set!(0xFA, "NOP", nop, imp, AddressMode::Imp, 2);
        set!(0xFC, "NOP", nop, imp, AddressMode::Imp, 7);
        set!(0xFD, "SBC", sbc, abx, AddressMode::Abx, 4);
        set!(0xFE, "INC", inc, abx, AddressMode::Abx, 7);

        table
    }

    fn fetch(&mut self, bus: &Bus) -> u8 {
        if self.lookup[self.opcode as usize].addrmode != AddressMode::Imp {
            self.fetched = self.read(bus, self.addr_abs);
        }
        self.fetched
    }

    fn imp(&mut self, _bus: &mut Bus) -> u8 {
        self.fetched = self.a;
        0
    }

    fn imm(&mut self, _bus: &mut Bus) -> u8 {
        self.addr_abs = self.pc;
        self.pc = self.pc.wrapping_add(1);
        0
    }

    fn zp0(&mut self, bus: &mut Bus) -> u8 {
        self.addr_abs = self.read(bus, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;
        0
    }

    fn zpx(&mut self, bus: &mut Bus) -> u8 {
        self.addr_abs = self.read(bus, self.pc).wrapping_add(self.x) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;
        0
    }

    fn zpy(&mut self, bus: &mut Bus) -> u8 {
        self.addr_abs = self.read(bus, self.pc).wrapping_add(self.y) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;
        0
    }

    fn rel(&mut self, bus: &mut Bus) -> u8 {
        self.addr_rel = self.read(bus, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        if self.addr_rel & 0x80 != 0 {
            self.addr_rel |= 0xFF00;
        }
        0
    }

    fn abs(&mut self, bus: &mut Bus) -> u8 {
        let lo = self.read(bus, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.read(bus, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs = (hi << 8) | lo;
        0
    }

    fn abx(&mut self, bus: &mut Bus) -> u8 {
        let lo = self.read(bus, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.read(bus, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs = ((hi << 8) | lo).wrapping_add(self.x as u16);
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    fn aby(&mut self, bus: &mut Bus) -> u8 {
        let lo = self.read(bus, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.read(bus, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs = ((hi << 8) | lo).wrapping_add(self.y as u16);
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    fn ind(&mut self, bus: &mut Bus) -> u8 {
        let ptr_lo = self.read(bus, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let ptr_hi = self.read(bus, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let ptr = (ptr_hi << 8) | ptr_lo;
        if ptr_lo == 0x00FF {
            self.addr_abs = ((self.read(bus, ptr & 0xFF00) as u16) << 8) | self.read(bus, ptr) as u16;
        } else {
            self.addr_abs = ((self.read(bus, ptr + 1) as u16) << 8) | self.read(bus, ptr) as u16;
        }
        0
    }

    fn izx(&mut self, bus: &mut Bus) -> u8 {
        let t = self.read(bus, self.pc);
        self.pc = self.pc.wrapping_add(1);
        let lo = self.read(bus, (t.wrapping_add(self.x)) as u16 & 0x00FF) as u16;
        let hi = self.read(bus, (t.wrapping_add(self.x).wrapping_add(1)) as u16 & 0x00FF) as u16;
        self.addr_abs = (hi << 8) | lo;
        0
    }

    fn izy(&mut self, bus: &mut Bus) -> u8 {
        let t = self.read(bus, self.pc);
        self.pc = self.pc.wrapping_add(1);
        let lo = self.read(bus, t as u16 & 0x00FF) as u16;
        let hi = self.read(bus, t.wrapping_add(1) as u16 & 0x00FF) as u16;
        self.addr_abs = ((hi << 8) | lo).wrapping_add(self.y as u16);
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    fn adc(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.temp = self.a as u16 + self.fetched as u16 + self.get_flag(StatusFlags::C) as u16;
        self.set_flag(StatusFlags::C, self.temp > 0xFF);
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(
            StatusFlags::V,
            (!(self.a as u16 ^ self.fetched as u16) & ((self.a as u16) ^ self.temp)) & 0x80 != 0,
        );
        self.set_flag(StatusFlags::N, self.temp & 0x80 != 0);
        self.a = (self.temp & 0x00FF) as u8;
        1
    }

    fn sbc(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        let value = (self.fetched as u16) ^ 0x00FF;
        self.temp = self.a as u16 + value + self.get_flag(StatusFlags::C) as u16;
        self.set_flag(StatusFlags::C, self.temp & 0xFF00 != 0);
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(
            StatusFlags::V,
            (self.temp ^ self.a as u16) & (self.temp ^ value) & 0x80 != 0,
        );
        self.set_flag(StatusFlags::N, self.temp & 0x80 != 0);
        self.a = (self.temp & 0x00FF) as u8;
        1
    }

    fn and(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.a &= self.fetched;
        self.set_flag(StatusFlags::Z, self.a == 0);
        self.set_flag(StatusFlags::N, self.a & 0x80 != 0);
        1
    }

    fn asl(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.temp = (self.fetched as u16) << 1;
        self.set_flag(StatusFlags::C, self.temp & 0xFF00 != 0);
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlags::N, self.temp & 0x80 != 0);
        if self.lookup[self.opcode as usize].addrmode == AddressMode::Imp {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(bus, self.addr_abs, (self.temp & 0x00FF) as u8);
        }
        0
    }

    fn bcc(&mut self, _bus: &mut Bus) -> u8 {
        if self.get_flag(StatusFlags::C) == 0 {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            self.pc = self.addr_abs;
        }
        0
    }

    fn bcs(&mut self, _bus: &mut Bus) -> u8 {
        if self.get_flag(StatusFlags::C) == 1 {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            self.pc = self.addr_abs;
        }
        0
    }

    fn beq(&mut self, _bus: &mut Bus) -> u8 {
        if self.get_flag(StatusFlags::Z) == 1 {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            self.pc = self.addr_abs;
        }
        0
    }

    fn bit(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.temp = self.a as u16 & self.fetched as u16;
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlags::N, self.fetched & 0x80 != 0);
        self.set_flag(StatusFlags::V, self.fetched & 0x40 != 0);
        0
    }

    fn bmi(&mut self, _bus: &mut Bus) -> u8 {
        if self.get_flag(StatusFlags::N) == 1 {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            self.pc = self.addr_abs;
        }
        0
    }

    fn bne(&mut self, _bus: &mut Bus) -> u8 {
        if self.get_flag(StatusFlags::Z) == 0 {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            self.pc = self.addr_abs;
        }
        0
    }

    fn bpl(&mut self, _bus: &mut Bus) -> u8 {
        if self.get_flag(StatusFlags::N) == 0 {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            self.pc = self.addr_abs;
        }
        0
    }

    fn brk(&mut self, bus: &mut Bus) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        self.set_flag(StatusFlags::I, true);
        self.write(bus, 0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(bus, 0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.set_flag(StatusFlags::B, true);
        self.write(bus, 0x0100 + self.sp as u16, self.status.bits());
        self.sp = self.sp.wrapping_sub(1);
        self.set_flag(StatusFlags::B, false);
        self.pc = self.read(bus, 0xFFFE) as u16 | ((self.read(bus, 0xFFFF) as u16) << 8);
        0
    }

    fn bvc(&mut self, _bus: &mut Bus) -> u8 {
        if self.get_flag(StatusFlags::V) == 0 {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            self.pc = self.addr_abs;
        }
        0
    }

    fn bvs(&mut self, _bus: &mut Bus) -> u8 {
        if self.get_flag(StatusFlags::V) == 1 {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            self.pc = self.addr_abs;
        }
        0
    }

    fn clc(&mut self, _bus: &mut Bus) -> u8 {
        self.set_flag(StatusFlags::C, false);
        0
    }
    fn cld(&mut self, _bus: &mut Bus) -> u8 {
        self.set_flag(StatusFlags::D, false);
        0
    }
    fn cli(&mut self, _bus: &mut Bus) -> u8 {
        self.set_flag(StatusFlags::I, false);
        0
    }
    fn clv(&mut self, _bus: &mut Bus) -> u8 {
        self.set_flag(StatusFlags::V, false);
        0
    }

    fn cmp(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.temp = self.a as u16 - self.fetched as u16;
        self.set_flag(StatusFlags::C, self.a >= self.fetched);
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlags::N, self.temp & 0x80 != 0);
        1
    }

    fn cpx(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.temp = self.x as u16 - self.fetched as u16;
        self.set_flag(StatusFlags::C, self.x >= self.fetched);
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlags::N, self.temp & 0x80 != 0);
        0
    }

    fn cpy(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.temp = self.y as u16 - self.fetched as u16;
        self.set_flag(StatusFlags::C, self.y >= self.fetched);
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlags::N, self.temp & 0x80 != 0);
        0
    }

    fn dec(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.temp = self.fetched.wrapping_sub(1) as u16;
        self.write(bus, self.addr_abs, (self.temp & 0x00FF) as u8);
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlags::N, self.temp & 0x80 != 0);
        0
    }

    fn dex(&mut self, _bus: &mut Bus) -> u8 {
        self.x = self.x.wrapping_sub(1);
        self.set_flag(StatusFlags::Z, self.x == 0);
        self.set_flag(StatusFlags::N, self.x & 0x80 != 0);
        0
    }

    fn dey(&mut self, _bus: &mut Bus) -> u8 {
        self.y = self.y.wrapping_sub(1);
        self.set_flag(StatusFlags::Z, self.y == 0);
        self.set_flag(StatusFlags::N, self.y & 0x80 != 0);
        0
    }

    fn eor(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.a ^= self.fetched;
        self.set_flag(StatusFlags::Z, self.a == 0);
        self.set_flag(StatusFlags::N, self.a & 0x80 != 0);
        1
    }

    fn inc(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.temp = self.fetched.wrapping_add(1) as u16;
        self.write(bus, self.addr_abs, (self.temp & 0x00FF) as u8);
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlags::N, self.temp & 0x80 != 0);
        0
    }

    fn inx(&mut self, _bus: &mut Bus) -> u8 {
        self.x = self.x.wrapping_add(1);
        self.set_flag(StatusFlags::Z, self.x == 0);
        self.set_flag(StatusFlags::N, self.x & 0x80 != 0);
        0
    }

    fn iny(&mut self, _bus: &mut Bus) -> u8 {
        self.y = self.y.wrapping_add(1);
        self.set_flag(StatusFlags::Z, self.y == 0);
        self.set_flag(StatusFlags::N, self.y & 0x80 != 0);
        0
    }

    fn jmp(&mut self, _bus: &mut Bus) -> u8 {
        self.pc = self.addr_abs;
        0
    }

    fn jsr(&mut self, bus: &mut Bus) -> u8 {
        self.pc = self.pc.wrapping_sub(1);
        self.write(bus, 0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(bus, 0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.pc = self.addr_abs;
        0
    }

    fn lda(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.a = self.fetched;
        self.set_flag(StatusFlags::Z, self.a == 0);
        self.set_flag(StatusFlags::N, self.a & 0x80 != 0);
        1
    }

    fn ldx(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.x = self.fetched;
        self.set_flag(StatusFlags::Z, self.x == 0);
        self.set_flag(StatusFlags::N, self.x & 0x80 != 0);
        1
    }

    fn ldy(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.y = self.fetched;
        self.set_flag(StatusFlags::Z, self.y == 0);
        self.set_flag(StatusFlags::N, self.y & 0x80 != 0);
        1
    }

    fn lsr(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.set_flag(StatusFlags::C, self.fetched & 0x01 != 0);
        self.temp = (self.fetched >> 1) as u16;
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlags::N, self.temp & 0x80 != 0);
        if self.lookup[self.opcode as usize].addrmode == AddressMode::Imp {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(bus, self.addr_abs, (self.temp & 0x00FF) as u8);
        }
        0
    }

    fn nop(&mut self, _bus: &mut Bus) -> u8 {
        match self.opcode {
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1,
            _ => 0,
        }
    }

    fn ora(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.a |= self.fetched;
        self.set_flag(StatusFlags::Z, self.a == 0);
        self.set_flag(StatusFlags::N, self.a & 0x80 != 0);
        1
    }

    fn pha(&mut self, bus: &mut Bus) -> u8 {
        self.write(bus, 0x0100 + self.sp as u16, self.a);
        self.sp = self.sp.wrapping_sub(1);
        0
    }

    fn php(&mut self, bus: &mut Bus) -> u8 {
        self.write(
            bus,
            0x0100 + self.sp as u16,
            self.status.bits() | StatusFlags::B.bits() | StatusFlags::U.bits(),
        );
        self.set_flag(StatusFlags::B, false);
        self.set_flag(StatusFlags::U, false);
        self.sp = self.sp.wrapping_sub(1);
        0
    }

    fn pla(&mut self, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.a = self.read(bus, 0x0100 + self.sp as u16);
        self.set_flag(StatusFlags::Z, self.a == 0);
        self.set_flag(StatusFlags::N, self.a & 0x80 != 0);
        0
    }

    fn plp(&mut self, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.status = StatusFlags::from_bits_truncate(self.read(bus, 0x0100 + self.sp as u16));
        self.set_flag(StatusFlags::U, true);
        0
    }

    fn rol(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.temp = ((self.fetched as u16) << 1) | self.get_flag(StatusFlags::C) as u16;
        self.set_flag(StatusFlags::C, self.temp & 0xFF00 != 0);
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlags::N, self.temp & 0x80 != 0);
        if self.lookup[self.opcode as usize].addrmode == AddressMode::Imp {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(bus, self.addr_abs, (self.temp & 0x00FF) as u8);
        }
        0
    }

    fn ror(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.temp = ((self.get_flag(StatusFlags::C) as u16) << 7) | ((self.fetched as u16) >> 1);
        self.set_flag(StatusFlags::C, self.fetched & 0x01 != 0);
        self.set_flag(StatusFlags::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlags::N, self.temp & 0x80 != 0);
        if self.lookup[self.opcode as usize].addrmode == AddressMode::Imp {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(bus, self.addr_abs, (self.temp & 0x00FF) as u8);
        }
        0
    }

    fn rti(&mut self, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.status = StatusFlags::from_bits_truncate(self.read(bus, 0x0100 + self.sp as u16));
        self.status.remove(StatusFlags::B);
        self.status.remove(StatusFlags::U);

        self.sp = self.sp.wrapping_add(1);
        let lo = self.read(bus, 0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        let hi = self.read(bus, 0x0100 + self.sp as u16) as u16;
        self.pc = (hi << 8) | lo;
        0
    }

    fn rts(&mut self, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let lo = self.read(bus, 0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        let hi = self.read(bus, 0x0100 + self.sp as u16) as u16;
        self.pc = ((hi << 8) | lo).wrapping_add(1);
        0
    }

    fn sec(&mut self, _bus: &mut Bus) -> u8 {
        self.set_flag(StatusFlags::C, true);
        0
    }
    fn sed(&mut self, _bus: &mut Bus) -> u8 {
        self.set_flag(StatusFlags::D, true);
        0
    }
    fn sei(&mut self, _bus: &mut Bus) -> u8 {
        self.set_flag(StatusFlags::I, true);
        0
    }

    fn sta(&mut self, bus: &mut Bus) -> u8 {
        self.write(bus, self.addr_abs, self.a);
        0
    }
    fn stx(&mut self, bus: &mut Bus) -> u8 {
        self.write(bus, self.addr_abs, self.x);
        0
    }
    fn sty(&mut self, bus: &mut Bus) -> u8 {
        self.write(bus, self.addr_abs, self.y);
        0
    }

    fn tax(&mut self, _bus: &mut Bus) -> u8 {
        self.x = self.a;
        self.set_flag(StatusFlags::Z, self.x == 0);
        self.set_flag(StatusFlags::N, self.x & 0x80 != 0);
        0
    }

    fn tay(&mut self, _bus: &mut Bus) -> u8 {
        self.y = self.a;
        self.set_flag(StatusFlags::Z, self.y == 0);
        self.set_flag(StatusFlags::N, self.y & 0x80 != 0);
        0
    }

    fn tsx(&mut self, _bus: &mut Bus) -> u8 {
        self.x = self.sp;
        self.set_flag(StatusFlags::Z, self.x == 0);
        self.set_flag(StatusFlags::N, self.x & 0x80 != 0);
        0
    }

    fn txa(&mut self, _bus: &mut Bus) -> u8 {
        self.a = self.x;
        self.set_flag(StatusFlags::Z, self.a == 0);
        self.set_flag(StatusFlags::N, self.a & 0x80 != 0);
        0
    }

    fn txs(&mut self, _bus: &mut Bus) -> u8 {
        self.sp = self.x;
        0
    }

    fn tya(&mut self, _bus: &mut Bus) -> u8 {
        self.a = self.y;
        self.set_flag(StatusFlags::Z, self.a == 0);
        self.set_flag(StatusFlags::N, self.a & 0x80 != 0);
        0
    }

    fn xxx(&mut self, _bus: &mut Bus) -> u8 {
        0
    }
}
