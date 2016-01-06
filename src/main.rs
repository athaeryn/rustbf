#[derive(Debug)]
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

fn main() {
    // Prints 'Hello, world!'
    // From https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages
    let program = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";

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

        match instruction {
            Some(i) => println!("{:?}", i),
            None => {}
        }
    }
}
