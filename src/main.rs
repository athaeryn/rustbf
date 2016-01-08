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

struct Tape {
    cells: [i32; 30000],
    ptr: i32
}

impl Tape {
    fn new() -> Tape {
        Tape { cells: [0; 30000], ptr: 0 }
    }

    fn increment_pointer(&mut self) {
        if self.ptr < 30000 {
            self.ptr += 1;
        }
    }

    fn decrement_pointer(&mut self) {
        if self.ptr > 0 {
            self.ptr -= 1;
        }
    }

    fn increment_byte(&mut self) {
        self.cells[self.ptr as usize] += 1
    }

    fn decrement_byte(&mut self) {
        self.cells[self.ptr as usize] -= 1
    }

    fn output_byte(&self) -> char {
        self.cells[self.ptr as usize] as u8 as char
    }

    fn input_byte(&mut self, byte: u8) {
        self.cells[self.ptr as usize] = byte as i32
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
    let mut tape = Tape::new();
    let mut cmd_ptr = 0;

    // TODO: Use a stack or something to support nested loops.
    let mut last_forward_jump = 0;

    loop {
        match commands[cmd_ptr] {
            Command::IncrementPointer => tape.increment_pointer(),
            Command::DecrementPointer => tape.decrement_pointer(),
            Command::IncrementByte => tape.increment_byte(),
            Command::DecrementByte => tape.decrement_byte(),
            Command::OutputByte => print!("{}", tape.output_byte()),
            Command::InputByte => { /* TODO */ },
            Command::JumpForward => {
                last_forward_jump = cmd_ptr;
                if tape.output_byte() as u8 == 0 {
                    loop {
                        cmd_ptr += 1;
                        if commands[cmd_ptr] != Command::JumpBackward {
                            break
                        }
                        if cmd_ptr >= commands.len() {
                            break
                        }
                    }
                }
            },
            Command::JumpBackward => {
                if tape.output_byte() as u8 != 0 {
                    cmd_ptr = last_forward_jump;
                }
            }
        }
        cmd_ptr += 1;
        if cmd_ptr >= commands.len() {
            break
        }
    }
}

