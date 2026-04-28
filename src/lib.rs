mod cpu;
mod bus;


use bus::Bus;
use cpu::Cpu;

pub struct System {
    pub bus: Bus,
    cpu: Cpu
}

