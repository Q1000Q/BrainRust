use std::env;
use std::fs::{self};

mod vanilla;

struct Operations {
    tape: [u8; 30000],
    pointer: usize,
    code: String,
    pc: usize,
}

impl Operations {
    fn execute(&mut self) {
        let mut tape = self.tape;
        let mut pointer = self.pointer;
        let code = &self.code;
        let mut pc = self.pc;
        
        let code_bytes = code.as_bytes();
        while pc < code.len() {
            match code_bytes[pc] {
                // '>' adds 1 to pointer (moves pointer 1 forward)
                b'>' => vanilla::forward(&tape, &mut pointer),
                // '<' removes 1 to pointer (moves pointer 1 backward)
                b'<' => vanilla::backwards(&tape, &mut pointer),
                // '+' adds 1 to localization at tape, that pointer points
                b'+' => vanilla::increment(&mut tape, pointer),
                // '-' removes 1 to localization at tape, that pointer points
                b'-' => vanilla::decrement(&mut tape, pointer),
                // '.' prints out content of tape cell that pointer is pointing (as a ASCII character)
                b'.' => vanilla::print(&tape, pointer),
                // ',' require user to enter a ASCII character and saves it to the tape cell that pointer is pointing
                b',' => vanilla::read(&mut tape, pointer),
                // '[' starts loop, if the value at the current cell is 0, then skips to the corresponding ']'. Otherwise, move to the next instruction
                b'[' => vanilla::loop_open(&tape, pointer, &mut pc, code_bytes),
                // ']' ends loop, f the value at the current cell is 0, move to the next instruction. Otherwise, move backwards in the instructions to the corresponding '['
                b']' => vanilla::loop_close(&tape, pointer, &mut pc, code_bytes),
                _ => (),
            }
            pc += 1;
            if pc >= code.len() { break; }
        }
    }
}

fn main() {
    // Reads Brainfuck code file from argument
    let args: Vec<String> = env::args().collect();
    let mut file_path: Option<String> = None;
    for arg in args {
        match arg.as_str() {
            _ => file_path = Some(arg),
        }
    }

    let file_path = file_path.expect("Usage: BrainRust.exe <file_path>");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read file");

    let mut main = Operations {
        tape: [0; 30000],
        pointer: 0,
        code: contents,
        pc: 0,
    };
    main.execute();
    
}