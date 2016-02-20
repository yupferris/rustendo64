use super::cpu;
use super::interconnect;

#[derive(Debug)]
pub struct N64 {
    cpu: cpu::Cpu
}

impl N64 {
    pub fn new(pif_rom: Box<[u8]>) -> N64 {
        let interconnect = interconnect::Interconnect::new(pif_rom);
        let cpu = cpu::Cpu::new(interconnect);

        N64 {
            cpu: cpu,
        }
    }

    // TODO: Better interface
    pub fn run(&mut self) {
        self.cpu.run();
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction();
    }
}
