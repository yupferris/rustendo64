pub struct Rdp;

impl Rdp {
    pub fn read_status_reg(&self) -> u32 {
        // TODO: Proper impl
        0
    }

    pub fn write_status_reg(&mut self, value: u32) {
        // TODO
        panic!("Write to rdp status reg: {:#?}", value);
    }
}
