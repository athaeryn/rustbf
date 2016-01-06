use std::fmt;

enum Command {
    Increment,
    Decrement,
    PointerLeft,
    PointerRight,
    JumpAhead,
    JumpBack,
    Read,
    Write
}

impl fmt::Display for Command {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Increment    => write!(f, "+"),
            Command::Decrement    => write!(f, "-"),
            Command::JumpAhead    => write!(f, "["),
            Command::JumpBack     => write!(f, "]"),
            Command::PointerRight => write!(f, ">"),
            Command::PointerLeft  => write!(f, "<"),
            Command::Read         => write!(f, ","),
            Command::Write        => write!(f, ".")
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
        let instruction: Option<Command>;

        instruction = match c {
            '+' => Some(Command::Increment),
            '-' => Some(Command::Decrement),
            '[' => Some(Command::JumpAhead),
            ']' => Some(Command::JumpBack),
            '>' => Some(Command::PointerRight),
            '<' => Some(Command::PointerLeft),
            ',' => Some(Command::Read),
            '.' => Some(Command::Write),
            _ => None
        };

        if let Some(i) = instruction {
            print!("{}", i);
        }
    }
}
