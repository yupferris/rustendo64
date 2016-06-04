use super::{Cpu, Interconnect};

#[derive(Debug)]
pub struct N64 {
    cpu: Cpu,
    interconnect: Interconnect,
}

impl N64 {
    pub fn new(boot_rom: Box<[u8]>, cart_rom: Box<[u8]>) -> N64 {
        N64 {
            cpu: Cpu::new(),
            interconnect: Interconnect::new(boot_rom, cart_rom),
        }
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn interconnect(&self) -> &Interconnect {
        &self.interconnect
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.interconnect);
    }
}
