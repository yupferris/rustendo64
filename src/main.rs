extern crate byteorder;
#[macro_use]
extern crate clap;

mod n64;
mod cpu;
mod interconnect;

use std::fs;
use std::io::Read;
use std::path::Path;
use clap::{App, Arg, ArgMatches};

fn get_arguments<'a>() -> ArgMatches<'a> {
    App::new("rustendo64")
        .version(crate_version!())
        .author("ferris <jake@fusetools.com>")
        .about("Livecoding a Nintendo 64 emulator in Rust :D")
        .arg(Arg::with_name("pif")
                 .help("Sets the PIF ROM needed for booting")
                 .takes_value(true)
                 .required(true))
        .arg(Arg::with_name("rom")
                 .help("Sets the ROM to run")
                 .takes_value(true)
                 .required(true))
        .get_matches()
}

fn main() {
    let arguments = get_arguments();
    let pif_file_name = arguments.value_of("pif").unwrap();
    let rom_file_name = arguments.value_of("rom").unwrap();

    let pif = read_bin(pif_file_name);
    let rom = read_bin(rom_file_name);

    let mut n64 = n64::N64::new(pif);
    n64.power_on_reset();
    loop {
        println!("N64: {:#?}", &n64);
        n64.run_instruction();
    }
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
