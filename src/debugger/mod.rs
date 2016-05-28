mod command;

use n64::*;

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
            print!("n64> ");
            stdout().flush().unwrap();

            let input = read_stdin();
            let command = input.parse();

            match command {
                Ok(Command::Step) => self.n64.run_instruction(),
                _ => println!("Invalid input")
            }
        }
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().into();
}
