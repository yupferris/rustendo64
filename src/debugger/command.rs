use std::str::FromStr;

pub enum Command {
    Step,
    Exit
}

impl FromStr for Command {
    // TODO: Proper error type
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "step" | "s" => Ok(Command::Step),
            "exit" | "quit" | "e" | "q" => Ok(Command::Exit),
            _ => Err(())
        }
    }
}
