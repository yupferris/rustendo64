use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum Command {
    Step,
    Exit,
    Repeat,
}

impl FromStr for Command {
    // TODO: Proper error type
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Command::Repeat),
            "step" | "s" => Ok(Command::Step),
            "exit" | "quit" | "e" | "q" => Ok(Command::Exit),
            _ => Err(()),
        }
    }
}
