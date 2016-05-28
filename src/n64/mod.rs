mod n64;
pub mod cpu;
mod pif;
mod rsp;
mod rdp;
mod audio_interface;
mod video_interface;
mod peripheral_interface;
mod serial_interface;
mod interconnect;
pub mod mem_map;

pub use self::n64::N64;
