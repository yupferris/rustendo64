const PIF_ROM_START: u32 =          0x1fc0_0000;
const PIF_ROM_LENGTH: u32 =         0x0000_07c0;
const PIF_ROM_END: u32 =            PIF_ROM_START + PIF_ROM_LENGTH - 1;

const PIF_RAM_START: u32 =          0x1fc0_07c0;
pub const PIF_RAM_LENGTH: u32 =     0x0000_0040;
const PIF_RAM_END: u32 =            PIF_RAM_START + PIF_RAM_LENGTH - 1;

const CART_DOM1_ADDR2_START: u32 =  0x1000_0000;
const CART_DOM1_ADDR2_LENGTH: u32 = 0x0fc0_0000;
const CART_DOM1_ADDR2_END: u32 =    CART_DOM1_ADDR2_START + CART_DOM1_ADDR2_LENGTH - 1;

const SP_DMEM_START: u32 =          0x0400_0000;
pub const SP_DMEM_LENGTH: u32 =     0x0000_1000;
const SP_DMEM_END: u32 =            SP_DMEM_START + SP_DMEM_LENGTH - 1;

const SP_IMEM_START: u32 =          0x0400_1000;
pub const SP_IMEM_LENGTH: u32 =     0x0000_1000;
const SP_IMEM_END: u32 =            SP_IMEM_START + SP_IMEM_LENGTH - 1;

const SP_BASE_REG: u32 =            0x0404_0000;
const SP_STATUS_REG: u32 =          0x0404_0010;
const SP_DMA_BUSY_REG: u32 =        0x0404_0018;

const DPC_BASE_REG: u32 =           0x0410_0000;
const DPC_STATUS_REG: u32 =         0x0410_000c;

const AI_BASE_REG: u32 =            0x0450_0000;
const AI_DRAM_ADDR_REG: u32 =       0x0450_0000;
const AI_LEN_REG: u32 =             0x0450_0004;

const VI_BASE_REG: u32 =            0x0440_0000;
const VI_INTR_REG: u32 =            0x0440_000c;
const VI_CURRENT_REG: u32 =         0x0440_0010;
const VI_H_START_REG: u32 =         0x0440_0024;

const PI_BASE_REG: u32 =            0x0460_0000;
const PI_STATUS_REG: u32 =          0x0460_0010;
const PI_BSD_DOM1_LAT_REG: u32 =    0x0460_0014;
const PI_BSD_DOM1_PWD_REG: u32 =    0x0460_0018;
const PI_BSD_DOM1_PGS_REG: u32 =    0x0460_001c;
const PI_BSD_DOM1_RLS_REG: u32 =    0x0460_0020;

const SI_BASE_REG: u32 =            0x0480_0000;
const SI_STATUS_REG: u32 =          0x0480_0018;

#[derive(Debug)]
pub enum Addr {
    PifRom(u32),
    PifRam(u32),

    CartDom1(u32),

    SpDmem(u32),
    SpImem(u32),

    SpStatusReg,
    SpDmaBusyReg,

    DpcStatusReg,

    AiDramAddrReg,
    AiLenReg,

    ViIntrReg,
    ViCurrentReg,
    ViHStartReg,

    PiStatusReg,
    PiBsdDom1LatReg,
    PiBsdDom1PwdReg,
    PiBsdDom1PgsReg,
    PiBsdDom1RlsReg,

    SiStatusReg,
}

pub fn map_addr(addr: u32) -> Addr {
    match addr {
        PIF_ROM_START ... PIF_ROM_END =>
            Addr::PifRom(addr - PIF_ROM_START),
        PIF_RAM_START ... PIF_RAM_END =>
            Addr::PifRam(addr - PIF_RAM_START),

        CART_DOM1_ADDR2_START ... CART_DOM1_ADDR2_END =>
            Addr::CartDom1(addr - CART_DOM1_ADDR2_START),

        SP_DMEM_START ... SP_DMEM_END =>
            Addr::SpDmem(addr - SP_DMEM_START),

        SP_IMEM_START ... SP_IMEM_END =>
            Addr::SpImem(addr - SP_IMEM_START),

        SP_STATUS_REG => Addr::SpStatusReg,
        SP_DMA_BUSY_REG => Addr::SpDmaBusyReg,

        DPC_STATUS_REG => Addr::DpcStatusReg,

        AI_DRAM_ADDR_REG => Addr::AiDramAddrReg,
        AI_LEN_REG => Addr::AiLenReg,

        VI_INTR_REG => Addr::ViIntrReg,
        VI_CURRENT_REG => Addr::ViCurrentReg,
        VI_H_START_REG => Addr::ViHStartReg,

        PI_STATUS_REG => Addr::PiStatusReg,
        PI_BSD_DOM1_LAT_REG => Addr::PiBsdDom1LatReg,
        PI_BSD_DOM1_PWD_REG => Addr::PiBsdDom1PwdReg,
        PI_BSD_DOM1_PGS_REG => Addr::PiBsdDom1PgsReg,
        PI_BSD_DOM1_RLS_REG => Addr::PiBsdDom1RlsReg,

        SI_STATUS_REG => Addr::SiStatusReg,

        _ => panic!("Unrecognized physical address: {:#x}", addr),
    }
}
