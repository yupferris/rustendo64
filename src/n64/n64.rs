use super::cpu::*;
use super::interconnect::*;

#[derive(Debug)]
pub struct N64 {
    cpu: Cpu
}

impl N64 {
    pub fn new(boot_rom: Box<[u8]>, cart_rom: Box<[u8]>) -> N64 {
        let interconnect = Interconnect::new(boot_rom, cart_rom);
        let cpu = Cpu::new(interconnect);

        N64 {
            cpu: cpu,
        }
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }
}
