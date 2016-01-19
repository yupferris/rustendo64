use super::cpu;
use super::interconnect;

pub struct N64 {
    cpu: cpu::Cpu
}

impl N64 {
    pub fn new(pif_rom: Vec<u8>) -> N64 {
        let interconnect = interconnect::Interconnect::new(pif_rom);
        let cpu = cpu::Cpu::new(interconnect);

        N64 {
            cpu: cpu,
        }
    }

    pub fn power_on_reset(&mut self) {
        self.cpu.power_on_reset();
    }

    // TODO: Better interface
    pub fn run(&mut self) {
        self.cpu.run();
    }
}
