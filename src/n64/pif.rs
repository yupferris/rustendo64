use byteorder::{BigEndian, ByteOrder};

use super::mem_map::PIF_RAM_LENGTH;

pub struct Pif {
    boot_rom: Box<[u8]>,
    ram: Box<[u8]>,
}

impl Pif {
    pub fn new(boot_rom: Box<[u8]>) -> Pif {
        Pif {
            boot_rom: boot_rom,

            ram: vec![0; PIF_RAM_LENGTH as usize].into_boxed_slice(),
        }
    }

    pub fn read_boot_rom(&self, addr: u32) -> u32 {
        BigEndian::read_u32(&self.boot_rom[addr as usize..])
    }

    pub fn read_ram(&self, addr: u32) -> u32 {
        BigEndian::read_u32(&self.ram[addr as usize..])
    }

    pub fn write_ram(&mut self, addr: u32, value: u32) {
        BigEndian::write_u32(&mut self.ram[addr as usize..], value);
    }
}
