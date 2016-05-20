#![deny(trivial_casts, trivial_numeric_casts)]
extern crate byteorder;

extern crate num;
extern crate core;

#[macro_use]
extern crate enum_primitive;

mod n64;
mod cpu;
mod pif;
mod rsp;
mod rdp;
mod audio_interface;
mod video_interface;
mod peripheral_interface;
mod serial_interface;
mod interconnect;
mod mem_map;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::io::ErrorKind;
use core::convert::AsRef;

fn main() {
    let pif_file_name_opt = env::args().nth(1);
    let rom_file_name_opt = env::args().nth(2);

    let (pif_file_name, rom_file_name) = 
	    match (pif_file_name_opt, rom_file_name_opt) {
	        (Some(pif), Some(rom)) => (pif, rom),
	        _ => {print_usage(); return}
	    };

    let (pif, rom) = 
	    match (read_bin(pif_file_name), read_bin(rom_file_name)) {
	        (Some(pif), Some(rom)) => (pif, rom),
	        _ => return,
	    };
    
    let mut n64 = n64::N64::new(pif, rom);
    loop {
        //println!("N64: {:#?}", &n64);
        n64.run_instruction();
    }
}

fn print_usage() {
    println!("Not enough arguments! Usage:");
    println!("rustendo64 <pif rom> <game rom>");
}

fn read_bin<P: AsRef<Path>>(path: P) -> Option<Box<[u8]>> {

    let mut file = match fs::File::open(&path) {
        Ok(v) => v,
        Err(e) => {
            println!("Unable to open file \"{}\" from current directory \"{}\": {}",
		             path.as_ref().display(), 
		             Path::new(".").canonicalize().unwrap().display(),
		             e);
            return None;
        }
    };
	
	// Create a box without flooding the stack.
	let pif_size = mem_map::PIF_ROM_LENGTH as usize;
	let mut temp_vec: Vec<u8> = Vec::with_capacity(pif_size);
    return match file.read_to_end(&mut temp_vec) {
        Ok(_) => Some(temp_vec.into_boxed_slice()),
        Err(e) => {
            match e.kind() {
	            	ErrorKind::UnexpectedEof => {
	            		println!("Error reading pif rom: file is too small.");
	            	},
	            	e => {
	            		println!("IO Error while reading pif rom: {:?}", e);
	            	}
            };
            None
        }
    }

}
