use byteorder::{BigEndian, ByteOrder};
use memmap::*;

use std::fmt;

const PIF_ROM_SIZE: usize = 2048;

const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    pif_rom: Vec<u8>,

    ram: Vec<u16>
}

impl Interconnect {
    pub fn new(pif_rom: Vec<u8>) -> Interconnect {
        Interconnect {
            pif_rom: pif_rom,

            ram: vec![0; RAM_SIZE]
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        match addr {
            PIF_ROM_START ... PIF_ROM_END => {
                let rel_addr = addr - PIF_ROM_START;
                BigEndian::read_u32(&self.pif_rom[rel_addr as usize..])
            },
            _ => {
                // TODO
                panic!("Unrecognized physical address: {:#x}", addr);
            }
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO: Impl Debug for Interconnect")
    }
}
