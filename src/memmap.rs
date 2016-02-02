// Constants for all the memory regions and registers.

// As long as we don't use most of these constants, avoid a ton of warnings.
#![allow(dead_code)]

macro_rules! define_consts {
    ($name:ident = $value:expr, $($nm:tt = $vl:tt),+) => {
        pub const $name: u32 = $value;
        define_consts!($($nm = $vl),+);
    };
    ($name:ident = $value:expr) => { pub const $name: u32 = $value; };
}

macro_rules! define_registers {
    ($base:expr, $name:ident, $($nm:tt),+) => {
        pub const $name: u32 = $base;
        define_registers!($base + 4, $($nm),+);
    };
    ($base:expr, $name:ident) => { pub const $name: u32 = $base; };
}

// RDRAM -----------------------------------------------------------------------

define_consts!(
    RDRAM_START     = 0x0000_0000,
    RDRAM_END       = 0x007f_ffff   // 8MB assumed
);

define_registers!(
    0x03f0_0000,
    RDRAM_REG_CONFIG,
    RDRAM_REG_DEVICE_ID,
    RDRAM_REG_DELAY,
    RDRAM_REG_MODE,
    RDRAM_REG_REF_INTERVAL,
    RDRAM_REG_REF_ROW,
    RDRAM_REG_RAS_INTERVAL,
    RDRAM_REG_MIN_INTERVAL,
    RDRAM_REG_ADDR_SELECT,
    RDRAM_REG_DEVICE_MANUF
);

define_registers!(
    0x0470_0000,
    RI_REG_MODE,
    RI_REG_CONFIG,
    RI_REG_CURRENT_LOAD,  // write only
    RI_REG_SELECT,
    RI_REG_REFRESH,
    RI_REG_LATENCY,
    RI_REG_RERROR,        // read only
    RI_REG_WERROR         // write only
);

// RSP interface ---------------------------------------------------------------

define_consts!(
    SP_DMEM_START = 0x0400_0000,
    SP_DMEM_END   = 0x0400_0fff,
    SP_IMEM_START = 0x0400_1000,
    SP_IMEM_END   = 0x0400_1fff
);

define_registers!(
    0x0404_0000,
    SP_REG_MEM_ADDR,
    SP_REG_DRAM_ADDR,
    SP_REG_RD_LEN,
    SP_REG_WR_LEN,
    SP_REG_STATUS,
    SP_REG_DMA_FULL,  // read only
    SP_REG_DMA_BUSY,  // read only
    SP_REG_SEMAPHORE
);

define_registers!(
    0x0408_0000,
    SP_REG_PC,
    SP_REG_IBIST
);

// RDP interface ---------------------------------------------------------------

define_registers!(
    0x0410_0000,
    DPC_REG_START,
    DPC_REG_END,
    DPC_REG_CURRENT,  // read only
    DPC_REG_STATUS,
    DPC_REG_CLOCK,    // read only
    DPC_REG_BUFBUSY,  // read only
    DPC_REG_PIPEBUSY, // read only
    DPC_REG_TMEM      // read only
);

define_registers!(
    0x0420_0000,
    DPS_REG_TBIST,
    DPS_REG_TEST_MODE,
    DPS_REG_BUFTEST_ADDR,
    DPS_REG_BUFTEST_DATA
);

// MIPS interface --------------------------------------------------------------

define_registers!(
    0x0430_0000,
    MI_REG_MODE,
    MI_REG_VERSION,   // read only
    MI_REG_INTR,      // read only
    MI_REG_INTR_MASK
);

// Video interface -------------------------------------------------------------

define_registers!(
    0x0440_0000,
    VI_REG_STATUS,
    VI_REG_ORIGIN,
    VI_REG_H_WIDTH,
    VI_REG_V_INTR,
    VI_REG_CURRENT,
    VI_REG_BURST,
    VI_REG_V_SYNC,
    VI_REG_H_SYNC,
    VI_REG_LEAP,
    VI_REG_H_START,
    VI_REG_V_START,
    VI_REG_V_BURST,
    VI_REG_X_SCALE,
    VI_REG_Y_SCALE
);

define_consts!(  // aliases
    VI_REG_CONTROL = VI_REG_STATUS
);

// Audio interface -------------------------------------------------------------

define_registers!(
    0x0450_0000,
    AI_REG_DRAM_ADDR,
    AI_REG_LEN,
    AI_REG_CONTROL,   // write only
    AI_REG_STATUS,
    AI_REG_DACRATE,   // write only
    AI_REG_BITRATE    // write only
);

// Peripheral interface --------------------------------------------------------

define_consts!(
    PIF_ROM_START = 0x1fc0_0000,
    PIF_ROM_END   = 0x1fc0_07bf,
    PIF_RAM_START = 0x1fc0_07c0,
    PIF_RAM_END   = 0x1fc0_07ff
);

define_registers!(
    0x0460_0000,
    PI_REG_DRAM_ADDR,
    PI_REG_CART_ADDR,
    PI_REG_RD_LEN,
    PI_REG_WR_LEN,
    PI_REG_STATUS,
    PI_REG_BSD_DOM1_LAT,
    PI_REG_BSD_DOM1_PWD,
    PI_REG_BSD_DOM1_PGS,
    PI_REG_BSD_DOM1_RLS,
    PI_REG_BSD_DOM2_LAT,
    PI_REG_BSD_DOM2_PWD,
    PI_REG_BSD_DOM2_PGS,
    PI_REG_BSD_DOM2_RLS
);

// Serial interface ------------------------------------------------------------

define_registers!(
    0x0480_0000,
    SI_REG_DRAM_ADDR,
    SI_REG_PIF_ADDR_RD64B,  // write only
    _SI_REG_RESERVED1,
    _SI_REG_RESERVED2,
    SI_REG_PIF_ADDR_WR64B,  // write only
    _SI_REG_RESERVED3,
    SI_REG_STATUS
);

// Cartridge and disk drive ----------------------------------------------------

define_consts!(
    CART_START    = 0x1000_0000,
    CART_END      = 0x1fbf_ffff,
    DD_ROM_START  = 0x0600_0000,
    DD_ROM_END    = 0x063f_ffff
);
