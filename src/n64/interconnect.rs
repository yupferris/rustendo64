use byteorder::{BigEndian, ByteOrder};

use super::{AudioInterface, PeripheralInterface, Pif, Rdp, Rsp, SerialInterface, VideoInterface};
use super::mem_map::{self, Addr};

use std::fmt;

const RDRAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    pif: Pif,

    rsp: Rsp,
    rdp: Rdp,

    ai: AudioInterface,
    vi: VideoInterface,

    pi: PeripheralInterface,

    si: SerialInterface,

    cart_rom: Box<[u8]>,

    rdram: Box<[u16]>,
}

impl Interconnect {
    pub fn new(boot_rom: Box<[u8]>, cart_rom: Box<[u8]>) -> Interconnect {
        Interconnect {
            pif: Pif::new(boot_rom),

            rdp: Rdp,
            rsp: Rsp::new(),

            ai: AudioInterface::default(),
            vi: VideoInterface::default(),

            pi: PeripheralInterface::default(),

            si: SerialInterface::default(),

            cart_rom: cart_rom,

            rdram: vec![0; RDRAM_SIZE].into_boxed_slice(),
        }
    }

    pub fn pif(&self) -> &Pif {
        &self.pif
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        match mem_map::map_addr(addr) {
            Addr::PifRom(offset) => self.pif.read_boot_rom(offset),
            Addr::PifRam(offset) => self.pif.read_ram(offset),

            Addr::CartDom1(offset) => BigEndian::read_u32(&self.cart_rom[offset as usize..]),

            Addr::SpDmem(offset) => self.rsp.read_dmem(offset),
            Addr::SpImem(offset) => self.rsp.read_imem(offset),

            Addr::SpStatusReg => self.rsp.read_status_reg(),
            Addr::SpDmaBusyReg => self.rsp.read_dma_busy_reg(),

            Addr::DpcStatusReg => self.rdp.read_status_reg(),

            Addr::AiDramAddrReg => self.ai.read_dram_addr_reg(),
            Addr::AiLenReg => self.ai.read_len_reg(),

            Addr::ViIntrReg => self.vi.read_intr_reg(),
            Addr::ViCurrentReg => self.vi.read_current_reg(),
            Addr::ViHStartReg => self.vi.read_h_start_reg(),

            Addr::PiStatusReg => self.pi.read_status_reg(),
            Addr::PiBsdDom1LatReg => self.pi.read_bsd_dom1_lat_reg(),
            Addr::PiBsdDom1PwdReg => self.pi.read_bsd_dom1_pwd_reg(),
            Addr::PiBsdDom1PgsReg => self.pi.read_bsd_dom1_pgs_reg(),
            Addr::PiBsdDom1RlsReg => self.pi.read_bsd_dom1_rls_reg(),

            Addr::SiStatusReg => self.si.read_status_reg(),
        }
    }

    pub fn write_word(&mut self, addr: u32, value: u32) {
        match mem_map::map_addr(addr) {
            Addr::PifRom(_) => panic!("Cannot write to PIF ROM"),
            Addr::PifRam(offset) => self.pif.write_ram(offset, value),

            Addr::CartDom1(offset) => panic!("Cannot write to cart ROM"),

            Addr::SpDmem(offset) => self.rsp.write_dmem(offset, value),
            Addr::SpImem(offset) => self.rsp.write_imem(offset, value),

            Addr::SpStatusReg => self.rsp.write_status_reg(value),
            Addr::SpDmaBusyReg => self.rsp.write_dma_busy_reg(value),

            Addr::DpcStatusReg => self.rdp.write_status_reg(value),

            Addr::AiDramAddrReg => self.ai.write_dram_addr_reg(value),
            Addr::AiLenReg => self.ai.write_len_reg(value),

            Addr::ViIntrReg => self.vi.write_intr_reg(value),
            Addr::ViCurrentReg => self.vi.write_current_reg(value),
            Addr::ViHStartReg => self.vi.write_h_start_reg(value),

            Addr::PiStatusReg => self.pi.write_status_reg(value),
            Addr::PiBsdDom1LatReg => self.pi.write_bsd_dom1_lat_reg(value),
            Addr::PiBsdDom1PwdReg => self.pi.write_bsd_dom1_pwd_reg(value),
            Addr::PiBsdDom1PgsReg => self.pi.write_bsd_dom1_pgs_reg(value),
            Addr::PiBsdDom1RlsReg => self.pi.write_bsd_dom1_rls_reg(value),

            Addr::SiStatusReg => self.si.write_status_reg(value),
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO: Impl Debug for Interconnect")
    }
}
