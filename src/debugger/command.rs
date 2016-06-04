use std::borrow::Cow;
use std::str::{self, FromStr};

use nom::{IResult, eof, space, digit};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Step(usize),
    Exit,
    Repeat,
}

impl FromStr for Command {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match command(s.as_bytes()) {
            IResult::Done(_, c) => Ok(c),
            err => Err(format!("Unable to parse command: {:?}", err).into())
        }
    }
}

named!(
    command<Command>,
    chain!(
        c: alt_complete!(
            step |
            exit |
            repeat) ~
            eof,
    || c));

named!(
    step<Command>,
    chain!(
        alt_complete!(tag!("step") | tag!("s")) ~
            count: opt!(preceded!(space, usize_parser)),
        || Command::Step(count.unwrap_or(1))));

named!(
    exit<Command>,
    map!(
        alt_complete!(tag!("exit") | tag!("quit") | tag!("e") | tag!("q")),
        |_| Command::Exit));

named!(
    repeat<Command>,
    value!(Command::Repeat));

named!(
    usize_parser<usize>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8),
        FromStr::from_str));
