use std::fmt;

#[derive(PartialEq)]
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
    let program = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++
                   .>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";

    let mut commands: Vec<Command> = Vec::new();

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
            commands.push(c);
        }
    }

    run(commands);
}

fn run(commands: Vec<Command>) {
    let mut tape = [0; 30000];
    let mut ptr = 0;

    let mut last_forward_jump = 0;
    let mut cmd_ptr = 0;

    loop {
        match commands[cmd_ptr] {
            Command::IncrementPointer => {
                if ptr < tape.len() { ptr += 1 };
                cmd_ptr += 1;
            },
            Command::DecrementPointer => {
                if ptr > 0 { ptr -= 1 };
                cmd_ptr += 1;
            },
            Command::IncrementByte => {
                tape[ptr] += 1;
                cmd_ptr += 1;
            },
            Command::DecrementByte => {
                tape[ptr] -= 1;
                cmd_ptr += 1;
            },
            Command::OutputByte => {
                let c = tape[ptr] as u8 as char;
                print!("{}", c);
                cmd_ptr += 1;
            },
            Command::InputByte => { cmd_ptr += 1 }, // TODO
            Command::JumpForward => {
                last_forward_jump = cmd_ptr;
                if tape[ptr] == 0 {
                    cmd_ptr += 1;
                    loop {
                        if commands[cmd_ptr] != Command::JumpBackward {
                            break
                        }
                        cmd_ptr += 1;
                        if cmd_ptr >= commands.len() {
                            break
                        }
                    }
                    cmd_ptr += 1;
                } else {
                    cmd_ptr += 1;
                }
            },
            Command::JumpBackward => {
                if tape[ptr] != 0 {
                    cmd_ptr = last_forward_jump;
                } else {
                    cmd_ptr += 1;
                }
            }
        }
        if cmd_ptr >= commands.len() {
            break
        }
    }
}

