mod cpu;
mod pif;
mod rsp;
mod rdp;
mod audio_interface;
mod video_interface;
mod peripheral_interface;
mod serial_interface;
mod interconnect;
mod mem_map;

use self::cpu::*;
use self::interconnect::*;

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

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction();
    }
}
