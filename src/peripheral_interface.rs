#[derive(Default)]
pub struct PeripheralInterface;

impl PeripheralInterface {
    pub fn read_status_reg(&self) -> u32 {
        // TODO: Proper impl
        0
    }

    pub fn write_status_reg(&mut self, value: u32) {
        if (value & (1 << 0)) != 0 {
            println!("WARNING: PI reset controller bit written not yet implemented");
        }

        if (value & (1 << 1)) != 0 {
            // TODO: Affect MI_INTR_REG
            println!("WARNING: PI clear intr bit written but not yet implemented");
        }
    }
}
