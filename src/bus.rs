

pub struct Bus {
    ram: Box<[u8; 64*1024]>
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            ram : vec![0; 64*1024].try_into().expect("Bus creation")
        }
    }
}

impl Bus {
    pub fn  write(&mut self, addr: u16, data: u8){
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr as usize] = data;
        }
    }

    pub fn read(&self, addr: u16) -> u8{
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr as usize]
        } else {
            0x00
        }
    }
}