const PIF_ROM_START: u32 =    0x1fc0_0000;
const PIF_ROM_LENGTH: u32 =   0x0000_07c0;
const PIF_ROM_END: u32 =      PIF_ROM_START + PIF_ROM_LENGTH - 1;

const SP_IMEM_START: u32 =    0x0400_1000;
pub const SP_IMEM_LENGTH: u32 =   0x0000_1000;
const SP_IMEM_END: u32 =      SP_IMEM_START + SP_IMEM_LENGTH - 1;

const SP_BASE_REG: u32 =      0x0404_0000;
const SP_STATUS_REG: u32 =    0x0404_0010;
const SP_DMA_BUSY_REG: u32 =  0x0404_0018;

const AI_BASE_REG: u32 =      0x0450_0000;
const AI_DRAM_ADDR_REG: u32 = 0x0450_0000;
const AI_LEN_REG: u32 =       0x0450_0004;

const VI_BASE_REG: u32 =      0x0440_0000;
const VI_INTR_REG: u32 =      0x0440_000c;
const VI_CURRENT_REG: u32 =   0x0440_0010;
const VI_H_START_REG: u32 =   0x0440_0024;

const PI_BASE_REG: u32 =      0x0460_0000;
const PI_STATUS_REG: u32 =    0x0460_0010;

pub enum Addr {
    PifRom(u32),

    SpImem(u32),

    SpStatusReg,
    SpDmaBusyReg,

    AiDramAddrReg,
    AiLenReg,

    ViIntrReg,
    ViCurrentReg,
    ViHStartReg,

    PiStatusReg,
}

pub fn map_addr(addr: u32) -> Addr {
    match addr {
        PIF_ROM_START ... PIF_ROM_END =>
            Addr::PifRom(addr - PIF_ROM_START),

        SP_IMEM_START ... SP_IMEM_END =>
            Addr::SpImem(addr - SP_IMEM_START),

        SP_STATUS_REG => Addr::SpStatusReg,
        SP_DMA_BUSY_REG => Addr::SpDmaBusyReg,

        AI_DRAM_ADDR_REG => Addr::AiDramAddrReg,
        AI_LEN_REG => Addr::AiLenReg,

        VI_INTR_REG => Addr::ViIntrReg,
        VI_CURRENT_REG => Addr::ViCurrentReg,
        VI_H_START_REG => Addr::ViHStartReg,

        PI_STATUS_REG => Addr::PiStatusReg,

        _ => panic!("Unrecognized physical address: {:#x}", addr)
    }
}
