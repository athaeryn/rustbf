use std::fmt;

enum Command {
    IncrementPointer,
    DecrementPointer,
    IncrementByte,
    DecrementByte,
    OutputByte,
    InputByte,
    JumpForward,
    JumpBackward
}

impl fmt::Display for Command {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::IncrementPointer => write!(f, ">"),
            Command::DecrementPointer => write!(f, "<"),
            Command::IncrementByte => write!(f, "+"),
            Command::DecrementByte => write!(f, "-"),
            Command::OutputByte => write!(f, "."),
            Command::InputByte => write!(f, ","),
            Command::JumpForward => write!(f, "["),
            Command::JumpBackward => write!(f, "]")
        }
    }
}


fn main() {
    // Prints 'Hello, world!'
    // From https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages
    let program = "
        ++++++++++
        [
            >+++++++
            >++++++++++
            >+++
            >+
            <<<<-
        ] end of the loop
        >++ .
        >+ .
        +++++++ .. +++ .
        >++ .
        << +++++++++++++++ .
        > . +++ . ------ . -------- .
        > + .
        >.
    ";

    for c in program.chars() {
        let command: Option<Command> = match c {
            '>' => Some(Command::IncrementPointer),
            '<' => Some(Command::DecrementPointer),
            '+' => Some(Command::IncrementByte),
            '-' => Some(Command::DecrementByte),
            '.' => Some(Command::OutputByte),
            ',' => Some(Command::InputByte),
            '[' => Some(Command::JumpForward),
            ']' => Some(Command::JumpBackward),
            _ => None
        };

        if let Some(c) = command {
            print!("{}", c);
        }
    }
}
