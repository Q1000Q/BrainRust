use std::env;
use std::fs::{self};

mod vanilla;
mod additiona_inputs;

struct Operations {
    tape: [u8; 30000],
    pointer: usize,
    code: String,
    pc: usize,
    vanilla: bool,
}

impl Operations {
    fn execute(&mut self) {        
        let code_bytes = self.code.as_bytes();
        while self.pc < self.code.len() {
            match code_bytes[self.pc] {
                // '>' adds 1 to pointer (moves pointer 1 forward)
                b'>' => vanilla::forward(&self.tape, &mut self.pointer),
                // '<' removes 1 to pointer (moves pointer 1 backward)
                b'<' => vanilla::backwards(&self.tape, &mut self.pointer),
                // '+' adds 1 to localization at tape, that pointer points
                b'+' => vanilla::increment(&mut self.tape, &self.pointer),
                // '-' removes 1 to localization at tape, that pointer points
                b'-' => vanilla::decrement(&mut self.tape, &self.pointer),
                // '.' prints out content of tape cell that pointer is pointing (as a ASCII character)
                b'.' => vanilla::print(&self.tape, &self.pointer),
                // ',' require user to enter a ASCII character and saves it to the tape cell that pointer is pointing
                b',' => vanilla::read(&mut self.tape, &self.pointer),
                // '[' starts loop, if the value at the current cell is 0, then skips to the corresponding ']'. Otherwise, move to the next instruction
                b'[' => vanilla::loop_open(&self.tape, &self.pointer, &mut self.pc, code_bytes),
                // ']' ends loop, f the value at the current cell is 0, move to the next instruction. Otherwise, move backwards in the instructions to the corresponding '['
                b']' => vanilla::loop_close(&self.tape, &self.pointer, &mut self.pc, code_bytes),

                // '\' sets the current cell to 10 (LFeed)
                b'\\' => if !self.vanilla { additiona_inputs::line_feeed_input(&mut self.tape, &self.pointer) },
                // b'c' sets the current cell to 'c' ASCII value
                b'b' => if !self.vanilla { additiona_inputs::byte_input(&mut self.tape, &self.pointer, &mut self.pc, code_bytes) },
                // s"abc" sets current cell to 'a' ASCII value, after that moves, and procedes to the next character (in this case 'b')
                b's' => if !self.vanilla { additiona_inputs::string_input(&mut self.tape, &mut self.pointer, &mut self.pc, &code_bytes) },
                // Numbers parsing (hex, dacimal or binary)
                b'0' => if !self.vanilla { additiona_inputs::number_input(&mut self.tape, &mut self.pointer, &mut self.pc, &code_bytes) },
                _ => (),
            }
            self.pc += 1;
            if self.pc >= self.code.len() { break; }
        }
    }
}

fn main() {
    // Reads Brainfuck code file from argument
    let args: Vec<String> = env::args().collect();
    let mut file_path: Option<String> = None;
    let mut vanilla: bool = false;
    for arg in args {
        match arg.as_str() {
            "-v" => vanilla = true,
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
        vanilla: vanilla,
    };
    main.execute();
    
}