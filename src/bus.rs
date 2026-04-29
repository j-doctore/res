pub struct Bus {
    pub ram: Box<[u8; 64 * 1024]>,
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            ram: vec![0; 64 * 1024].try_into().expect("Bus creation"),
        }
    }
}

impl Bus {
    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }

    pub fn read(&self, addr: u16, _read_only: bool) -> u8 {
        self.ram[addr as usize]
    }

    pub fn load_program(&mut self, program: &[u8], start_addr: u16) {
        let start = start_addr as usize;
        let end = (start + program.len()).min(self.ram.len());
        let len = end.saturating_sub(start);
        self.ram[start..end].copy_from_slice(&program[..len]);
    }

    pub fn set_reset_vector(&mut self, addr: u16) {
        self.ram[0xFFFC] = (addr & 0x00FF) as u8;
        self.ram[0xFFFD] = ((addr >> 8) & 0x00FF) as u8;
    }
}