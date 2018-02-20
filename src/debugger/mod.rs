mod command;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use rustyline::Editor;
use n64::cpu::Instruction;
use n64::cpu::opcode::Opcode::*;
use n64::mem_map;
use n64::mem_map::Addr::*;
use n64::N64;
use self::command::Command;

static HISTFILE: &'static str = ".config/rustendo64/debug_history";

pub struct Debugger<'e> {
    n64: N64,
    editor: Editor<'e>,
    history_path: Option<PathBuf>,
    last_command: Option<Command>,
}

impl<'e> Debugger<'e> {
    pub fn new(n64: N64) -> Debugger<'e> {
        let mut editor = Editor::new();
        let history_path = env::home_dir().map(|p| p.join(HISTFILE));
        if let Some(ref path) = history_path {
            let _ = editor.load_history(path);
        }
        Debugger {
            n64: n64,
            editor: editor,
            history_path: history_path,
            last_command: None,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.editor.readline("r64> ") {
                Err(_) => {
                    println!("Interrupt/Quit.");
                    process::exit(1);
                }
                Ok(input) => {
                    if input.len() > 0 {
                        self.editor.add_history_entry(&input);
                    }

                    let command = match (input.parse(), self.last_command) {
                        (Ok(Command::Repeat), Some(c)) => Ok(c),
                        (Ok(Command::Repeat), None) => Err("No last command".into()),
                        (Ok(c), _) => Ok(c),
                        (Err(e), _) => Err(e)
                    };

                    match command {
                        Ok(Command::Step(count)) => self.step(count),
                        Ok(Command::Exit) => {
                            if let Some(ref path) = self.history_path {
                                let _ = fs::create_dir_all(path.parent().unwrap());
                                let _ = self.editor.save_history(path);
                            }
                            break;
                        }
                        Ok(Command::Repeat) => unreachable!(),
                        Err(ref e) => println!("{}", e)
                    }
                    self.last_command = command.ok();
                }
            }
        }
    }

    pub fn step(&mut self, count: usize) {
        for _ in 0..count {
            let current_pc = self.n64.cpu().current_pc_phys();
            let addr = mem_map::map_addr(current_pc as u32);
            let instr = Instruction(match addr {
                PifRom(offset) => self.n64.interconnect().pif().read_boot_rom(offset),
                _ => panic!("Debugger can't inspect address: {:?}", addr),
            });

            print!("{:018X}: ", current_pc);

            match instr.opcode() {
                Special => print!("{:?} (Special)", instr.special_op()),
                RegImm => print!("{:?} (RegImm)", instr.reg_imm_op()),
                _ => print!("{:?}", instr),
            }

            if self.n64.cpu().will_execute_from_delay_slot() {
                println!(" (DELAY)");
            } else {
                println!("");
            }

            self.n64.step();
        }
    }
}
