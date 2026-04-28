use crate::bus::Bus;

pub enum Flag {
    C = (1 << 0), // Carry Bit
    Z = (1 << 1), // Zero
    I = (1 << 2), // Disable Interrupts
    D = (1 << 3), // Decimal Mode (unused in this implementation)
    B = (1 << 4), // Break
    U = (1 << 5), // Unused
    V = (1 << 6), // Overflow
    N = (1 << 7), // Negative
}

pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    status: u8,

    fetched: u8,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8,
    clock_count: u32,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            status: 0,
            fetched: 0,
            addr_abs: 0,
            addr_rel: 0,
            opcode: 0,
            cycles: 0,
            clock_count: 0,
        }
    }
}

impl Cpu {
    pub fn fetch() -> u8 {
        todo!()
    }

    fn getFlag(&self, flag: Flag) -> u8 {
        if flag as u8 & self.status > 0 { 1 } else { 0 }
    }
    // Sets or clears a specific bit of the status register
    fn setFlag(mut self, flag: Flag, val: bool) {
        //is this correct?
        self.status |= flag as u8 & val as u8
    }

    pub fn reset(mut self, bus: &Bus) {
        self.addr_abs = 0xFFFC;
        let lo = self.read(bus, self.addr_abs);
        let hi = self.read(bus, self.addr_abs + 1);

        self.pc = (hi << 8) as u16 | lo as u16;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.status = 0x00 | Flag::U as u8;

        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched = 0x00;

        self.cycles = 0;
    }

    pub fn irq() {
        todo!()
    }

    pub fn clock() {
        todo!()
    }

    pub fn complete() -> bool {
        todo!()
    }

    pub fn read(&self, bus: &Bus, addr: u16) -> u8 {
        bus.read(addr)
    }
    pub fn write(&mut self, bus: &mut Bus, addr: u16, data: u8) {
        bus.write(addr, data);
    }
}
