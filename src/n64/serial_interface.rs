#[derive(Default)]
pub struct SerialInterface;

impl SerialInterface {
    pub fn read_status_reg(&self) -> u32 {
        // TODO: Proper impl
        0
    }

    pub fn write_status_reg(&mut self, value: u32) {
        panic!("Writes to SI status reg not yet implemented");
    }
}
