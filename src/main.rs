use std::fmt;
use std::io::{stdin, Write, Read};

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

    fn output_byte(&self) -> u8 {
        self.cells[self.ptr as usize] as u8
    }

    fn input_byte(&mut self, byte: u8) {
        self.cells[self.ptr as usize] = byte as i32
    }
}

fn main() {
    // reimplemention of `cat`
    // from https://esolangs.org/wiki/Brainfuck#Cat
    let program = ",[.[-],]";

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

    let mut jump_stack: Vec<usize> = Vec::new();

    loop {
        match commands[cmd_ptr] {
            Command::IncrementPointer => tape.increment_pointer(),
            Command::DecrementPointer => tape.decrement_pointer(),
            Command::IncrementByte => tape.increment_byte(),
            Command::DecrementByte => tape.decrement_byte(),
            Command::OutputByte => { write_byte(tape.output_byte()); },
            Command::InputByte => { tape.input_byte(read_byte()); },
            Command::JumpForward => {
                let mut local_jump_depth = 1;
                jump_stack.push(cmd_ptr);
                if tape.output_byte() == 0 {
                    loop {
                        cmd_ptr += 1;
                        match commands[cmd_ptr] {
                            Command::JumpForward => {
                                local_jump_depth += 1;
                                jump_stack.push(cmd_ptr);
                            },
                            Command::JumpBackward => {
                                local_jump_depth -= 1;
                                jump_stack.pop().unwrap();
                                if local_jump_depth == 0 {
                                    break
                                }
                            },
                            _ => {}
                        }
                        if cmd_ptr >= commands.len() {
                            panic!("Ran out of instructions while jumping forward");
                        }
                    }
                }
            },
            Command::JumpBackward => {
                if tape.output_byte() != 0 {
                    match jump_stack.last() {
                        Some(ptr) => { cmd_ptr = *ptr },
                        None => panic!("Could not jump backward from ']' token at {}", cmd_ptr)
                    }
                } else {
                    // We're not jumping back, so pop the last location off the
                    // jump stack.
                    jump_stack.pop().unwrap();
                }
            }
        }
        cmd_ptr += 1;
        if cmd_ptr >= commands.len() {
            // We've reached the end of execution.
            break
        }
    }
}

fn write_byte(byte: u8) {
    print!("{}", byte as char);
}

fn read_byte() -> u8 {
    let mut buf = [0];
    let result = stdin().take(1).read(&mut buf);
    match result {
        Ok(x) => { if x > 0 { buf[0] } else { 0 } },
        Err(_) => { 0 }
    }
}
