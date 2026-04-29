use std::collections::BTreeMap;

use crate::{bus::Bus, cpu::Cpu, demo};

pub struct System {
    pub bus: Bus,
    pub cpu: Cpu,
    pub disassembly: BTreeMap<u16, String>,
}

impl Default for System {
    fn default() -> Self {
        let mut system = Self {
            bus: Bus::default(),
            cpu: Cpu::default(),
            disassembly: BTreeMap::new(),
        };
        system.load_demo();
        system
    }
}

impl System {
    pub fn load_demo(&mut self) {
        demo::load_demo_into_bus(&mut self.bus);
        self.cpu.reset(&self.bus);
        self.refresh_disassembly();
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&self.bus);
    }

    pub fn irq(&mut self) {
        self.cpu.irq(&mut self.bus);
    }

    pub fn nmi(&mut self) {
        self.cpu.nmi(&mut self.bus);
    }

    pub fn step_instruction(&mut self) {
        loop {
            self.cpu.clock(&mut self.bus);
            if self.cpu.complete() {
                break;
            }
        }
    }

    pub fn refresh_disassembly(&mut self) {
        self.disassembly = self.cpu.disassemble(&self.bus, 0x0000, 0xFFFF);
    }
}
