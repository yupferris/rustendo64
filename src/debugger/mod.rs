mod command;

use n64::*;
use n64::mem_map::*;
use n64::mem_map::Addr::*;

use self::command::*;

use std::io::*;

pub struct Debugger {
    n64: N64
}

impl Debugger {
    pub fn new(n64: N64) -> Debugger {
        Debugger {
            n64: n64
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("r64> ");
            stdout().flush().unwrap();

            let command = read_stdin().parse();
            match command {
                Ok(Command::Step) => self.step(),
                Ok(Command::Exit) => break,
                Err(_) => println!("Invalid input")
            }
        }
    }

    pub fn step(&mut self) {
        let current_pc = self.n64.cpu().current_pc_phys();
        let addr = map_addr(current_pc as u32);
        let instr = match addr {
            PifRom(offset) => 0, // TODO
            _ => panic!("Debugger can't inspect address: {:?}", addr)
        };

        println!("{:018X}", current_pc);

        /*match instr.opcode() {
            Special => print!("Special: {:?}", instr.special_op()),
            RegImm => print!("RegImm: {:?}", instr.reg_imm_op()),
            _ => print!("{:?}", instr)
        }
        match delay_slot {
            DelaySlot::Yes => println!(" (DELAY)"),
            _ => println!("")
        };*/

        self.n64.step();
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}
