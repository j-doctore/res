use std::collections::BTreeMap;

use bitflags::bitflags;

use crate::bus::Bus;

mod instr;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct StatusFlags: u8 {
        const C = 1 << 0;
        const Z = 1 << 1;
        const I = 1 << 2;
        const D = 1 << 3;
        const B = 1 << 4;
        const U = 1 << 5;
        const V = 1 << 6;
        const N = 1 << 7;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressMode {
    Imp,
    Imm,
    Zp0,
    Zpx,
    Zpy,
    Rel,
    Abs,
    Abx,
    Aby,
    Ind,
    Izx,
    Izy,
}

type OpFn = fn(&mut Cpu, &mut Bus) -> u8;
type AddrFn = fn(&mut Cpu, &mut Bus) -> u8;

#[derive(Clone, Copy)]
pub struct Instruction {
    pub name: &'static str,
    pub operate: OpFn,
    pub addrmode_fn: AddrFn,
    pub addrmode: AddressMode,
    pub cycles: u8,
}

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub status: StatusFlags,

    fetched: u8,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8,
    pub clock_count: u32,
    temp: u16,
    lookup: Vec<Instruction>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            status: StatusFlags::empty(),
            fetched: 0,
            addr_abs: 0,
            addr_rel: 0,
            opcode: 0,
            cycles: 0,
            clock_count: 0,
            temp: 0,
            lookup: Self::build_lookup(),
        }
    }

    pub fn read(&self, bus: &Bus, addr: u16) -> u8 {
        bus.read(addr, false)
    }

    pub fn write(&self, bus: &mut Bus, addr: u16, data: u8) {
        bus.write(addr, data);
    }

    pub fn get_flag(&self, flag: StatusFlags) -> u8 {
        if self.status.contains(flag) { 1 } else { 0 }
    }

    pub fn set_flag(&mut self, flag: StatusFlags, value: bool) {
        if value {
            self.status.insert(flag);
        } else {
            self.status.remove(flag);
        }
    }

    pub fn reset(&mut self, bus: &Bus) {
        self.addr_abs = 0xFFFC;
        let lo = self.read(bus, self.addr_abs) as u16;
        let hi = self.read(bus, self.addr_abs + 1) as u16;
        self.pc = (hi << 8) | lo;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.status = StatusFlags::U;

        self.addr_rel = 0;
        self.addr_abs = 0;
        self.fetched = 0;
        self.cycles = 8;
    }

    pub fn irq(&mut self, bus: &mut Bus) {
        if self.get_flag(StatusFlags::I) == 0 {
            self.write(bus, 0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
            self.sp = self.sp.wrapping_sub(1);
            self.write(bus, 0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
            self.sp = self.sp.wrapping_sub(1);

            self.set_flag(StatusFlags::B, false);
            self.set_flag(StatusFlags::U, true);
            self.set_flag(StatusFlags::I, true);
            self.write(bus, 0x0100 + self.sp as u16, self.status.bits());
            self.sp = self.sp.wrapping_sub(1);

            self.addr_abs = 0xFFFE;
            let lo = self.read(bus, self.addr_abs) as u16;
            let hi = self.read(bus, self.addr_abs + 1) as u16;
            self.pc = (hi << 8) | lo;

            self.cycles = 7;
        }
    }

    pub fn nmi(&mut self, bus: &mut Bus) {
        self.write(bus, 0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(bus, 0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);

        self.set_flag(StatusFlags::B, false);
        self.set_flag(StatusFlags::U, true);
        self.set_flag(StatusFlags::I, true);
        self.write(bus, 0x0100 + self.sp as u16, self.status.bits());
        self.sp = self.sp.wrapping_sub(1);

        self.addr_abs = 0xFFFA;
        let lo = self.read(bus, self.addr_abs) as u16;
        let hi = self.read(bus, self.addr_abs + 1) as u16;
        self.pc = (hi << 8) | lo;

        self.cycles = 8;
    }

    pub fn clock(&mut self, bus: &mut Bus) {
        if self.cycles == 0 {
            self.opcode = self.read(bus, self.pc);
            self.set_flag(StatusFlags::U, true);
            self.pc = self.pc.wrapping_add(1);

            let instruction = self.lookup[self.opcode as usize];
            self.cycles = instruction.cycles;

            let additional_cycle1 = (instruction.addrmode_fn)(self, bus);
            let additional_cycle2 = (instruction.operate)(self, bus);
            self.cycles = self.cycles.wrapping_add(additional_cycle1 & additional_cycle2);
            self.set_flag(StatusFlags::U, true);
        }

        self.clock_count = self.clock_count.wrapping_add(1);
        self.cycles = self.cycles.wrapping_sub(1);
    }

    pub fn complete(&self) -> bool {
        self.cycles == 0
    }

    pub fn disassemble(&self, bus: &Bus, start: u16, stop: u16) -> BTreeMap<u16, String> {
        let mut addr = start as u32;
        let mut map_lines = BTreeMap::new();

        let hex = |mut n: u32, d: usize| {
            let mut s = vec!['0'; d];
            for i in (0..d).rev() {
                s[i] = "0123456789ABCDEF".as_bytes()[(n & 0xF) as usize] as char;
                n >>= 4;
            }
            s.into_iter().collect::<String>()
        };

        while addr <= stop as u32 {
            let line_addr = addr as u16;
            let mut s_inst = format!("${}: ", hex(addr, 4));
            let opcode = bus.read(addr as u16, true);
            addr += 1;
            let instruction = self.lookup[opcode as usize];
            s_inst.push_str(instruction.name);
            s_inst.push(' ');

            match instruction.addrmode {
                AddressMode::Imp => s_inst.push_str(" {IMP}"),
                AddressMode::Imm => {
                    let value = bus.read(addr as u16, true);
                    addr += 1;
                    s_inst.push_str(&format!(" #${} {{IMM}}", hex(value as u32, 2)));
                }
                AddressMode::Zp0 => {
                    let lo = bus.read(addr as u16, true);
                    addr += 1;
                    s_inst.push_str(&format!(" ${} {{ZP0}}", hex(lo as u32, 2)));
                }
                AddressMode::Zpx => {
                    let lo = bus.read(addr as u16, true);
                    addr += 1;
                    s_inst.push_str(&format!(" ${}, X {{ZPX}}", hex(lo as u32, 2)));
                }
                AddressMode::Zpy => {
                    let lo = bus.read(addr as u16, true);
                    addr += 1;
                    s_inst.push_str(&format!(" ${}, Y {{ZPY}}", hex(lo as u32, 2)));
                }
                AddressMode::Izx => {
                    let lo = bus.read(addr as u16, true);
                    addr += 1;
                    s_inst.push_str(&format!(" (${}, X) {{IZX}}", hex(lo as u32, 2)));
                }
                AddressMode::Izy => {
                    let lo = bus.read(addr as u16, true);
                    addr += 1;
                    s_inst.push_str(&format!(" (${}), Y {{IZY}}", hex(lo as u32, 2)));
                }
                AddressMode::Abs => {
                    let lo = bus.read(addr as u16, true) as u16;
                    addr += 1;
                    let hi = bus.read(addr as u16, true) as u16;
                    addr += 1;
                    s_inst.push_str(&format!(" ${} {{ABS}}", hex(((hi << 8) | lo) as u32, 4)));
                }
                AddressMode::Abx => {
                    let lo = bus.read(addr as u16, true) as u16;
                    addr += 1;
                    let hi = bus.read(addr as u16, true) as u16;
                    addr += 1;
                    s_inst.push_str(&format!(" ${}, X {{ABX}}", hex(((hi << 8) | lo) as u32, 4)));
                }
                AddressMode::Aby => {
                    let lo = bus.read(addr as u16, true) as u16;
                    addr += 1;
                    let hi = bus.read(addr as u16, true) as u16;
                    addr += 1;
                    s_inst.push_str(&format!(" ${}, Y {{ABY}}", hex(((hi << 8) | lo) as u32, 4)));
                }
                AddressMode::Ind => {
                    let lo = bus.read(addr as u16, true) as u16;
                    addr += 1;
                    let hi = bus.read(addr as u16, true) as u16;
                    addr += 1;
                    s_inst.push_str(&format!(" (${}) {{IND}}", hex(((hi << 8) | lo) as u32, 4)));
                }
                AddressMode::Rel => {
                    let value = bus.read(addr as u16, true);
                    addr += 1;
                    let target = (addr as i32).wrapping_add(value as i8 as i32) as u16;
                    s_inst.push_str(&format!(" ${} [${}] {{REL}}", hex(value as u32, 2), hex(target as u32, 4)));
                }
            }

            map_lines.insert(line_addr, s_inst);
        }

        map_lines
    }

}
