#[derive(Default)]
pub struct PeripheralInterface;

impl PeripheralInterface {
    pub fn read_status_reg(&self) -> u32 {
        // TODO: Proper impl
        0
    }

    pub fn write_status_reg(&mut self, value: u32) {
        if (value & (1 << 0)) != 0 {
            println!("WARNING: PI reset controller bit written but not yet implemented");
        }

        if (value & (1 << 1)) != 0 {
            // TODO: Affect MI_INTR_REG
            println!("WARNING: PI clear intr bit written but not yet implemented");
        }
    }

    pub fn read_bsd_dom1_lat_reg(&self) -> u32 {
        // TODO: Proper impl (probably not necessary)
        0
    }

    pub fn write_bsd_dom1_lat_reg(&mut self, value: u32) {
        // TODO: Proper impl (probably not necessary)
        println!("PI_BSD_DOM1_LAT_REG written: {:#x}", value);
    }

    pub fn read_bsd_dom1_pwd_reg(&self) -> u32 {
        // TODO: Proper impl (probably not necessary)
        0
    }

    pub fn write_bsd_dom1_pwd_reg(&mut self, value: u32) {
        // TODO: Proper impl (probably not necessary)
        println!("PI_BSD_DOM1_PWD_REG written: {:#x}", value);
    }

    pub fn read_bsd_dom1_pgs_reg(&self) -> u32 {
        // TODO: Proper impl (probably not necessary)
        0
    }

    pub fn write_bsd_dom1_pgs_reg(&mut self, value: u32) {
        // TODO: Proper impl (probably not necessary)
        println!("PI_BSD_DOM1_PGS_REG written: {:#x}", value);
    }

    pub fn read_bsd_dom1_rls_reg(&self) -> u32 {
        // TODO: Proper impl (probably not necessary)
        0
    }

    pub fn write_bsd_dom1_rls_reg(&mut self, value: u32) {
        // TODO: Proper impl (probably not necessary)
        println!("PI_BSD_DOM1_RLS_REG written: {:#x}", value);
    }
}
