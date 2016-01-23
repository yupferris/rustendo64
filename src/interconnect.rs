use std::fmt;

const PIF_ROM_SIZE: usize = 2048;

const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    pif_rom: Vec<u8>,

    ram: Box<[u16]>,
}

impl Default for Interconnect {
    fn default() -> Self {
        Interconnect {
            pif_rom: Vec::new(),
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
        }
    }
}

impl Interconnect {
    pub fn new(pif_rom: Vec<u8>) -> Interconnect {
        Interconnect { pif_rom: pif_rom, ..Default::default() }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        // TODO: Replace constants with useful names
        if addr >= 0x1fc0_0000 && addr < 0x1fc0_07c0 {
            let rel_addr = addr - 0x1fc0_0000;
            // TODO: Check endianness
            // TODO: Check out byteorder crate
            ((self.pif_rom[(rel_addr + 0) as usize] as u32) << 24) |
            ((self.pif_rom[(rel_addr + 1) as usize] as u32) << 16) |
            ((self.pif_rom[(rel_addr + 2) as usize] as u32) <<  8) |
            ((self.pif_rom[(rel_addr + 3) as usize] as u32) <<  0)
        } else {
            // TODO
            panic!("Unrecognized address: {:#x}", addr);
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO: Impl Debug for Interconnect")
    }
}
