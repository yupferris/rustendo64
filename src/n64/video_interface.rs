#[derive(Default)]
pub struct VideoInterface {
    interrupt_half_line: u32,

    active_video_start: u32,
    active_video_end: u32,
}

impl VideoInterface {
    pub fn read_intr_reg(&self) -> u32 {
        self.interrupt_half_line
    }

    pub fn write_intr_reg(&mut self, value: u32) {
        self.interrupt_half_line = value & 0x0000_03ff;
    }

    pub fn read_current_reg(&self) -> u32 {
        // TODO: Proper impl
        0
    }

    pub fn write_current_reg(&mut self, value: u32) {
        // TODO: Clear interrupt
    }

    pub fn read_h_start_reg(&self) -> u32 {
        (self.active_video_start << 16) | self.active_video_end
    }

    pub fn write_h_start_reg(&mut self, value: u32) {
        self.active_video_start = (value >> 16) & 0x0000_03ff;
        self.active_video_end = value & 0x0000_03ff;
    }
}
