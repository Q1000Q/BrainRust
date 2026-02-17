use std::env;
use std::fs::{self};

mod vanilla;
mod additional_inputs;
mod additional_outputs;
mod file_operations;
mod extended;
mod extras;

fn execute(op: &mut Operations, pointer: Option<usize>, pc: Option<usize>) {
    let mut pointer: usize = pointer.unwrap_or(0);        
    let mut pc = pc.unwrap_or(0);   
         
    let vanilla = op.vanilla;
    let tape = &mut op.tape;
    let code = &op.code;

    let code_bytes = code.as_bytes();
    while pc < code.len() {
        match code_bytes[pc] {
            // '>' adds 1 to pointer (moves pointer 1 forward)
            b'>' => if vanilla { vanilla::forward(tape, &mut pointer) } else { extended::forward_extended(tape, &mut pointer, &mut pc, &code_bytes); },
            // '<' removes 1 to pointer (moves pointer 1 backward)
            b'<' => if vanilla { vanilla::backward(tape, &mut pointer) } else { extended::backward_extended(tape, &mut pointer, &mut pc, &code_bytes); },
            // '+' adds 1 to localization at tape, that pointer points
            b'+' => vanilla::increment(tape, &pointer),
            // '-' removes 1 to localization at tape, that pointer points
            b'-' => vanilla::decrement(tape, &pointer),
            // '.' prints out content of tape cell that pointer is pointing (as a ASCII character)
            b'.' => vanilla::print(tape, &pointer),
            // ',' require user to enter a ASCII character and saves it to the tape cell that pointer is pointing
            b',' => vanilla::read(tape, &pointer),
            // '[' starts loop, if the value at the current cell is 0, then skips to the corresponding ']'. Otherwise, move to the next instruction
            b'[' => vanilla::loop_open(tape, &pointer, &mut pc, code_bytes),
            // ']' ends loop, f the value at the current cell is 0, move to the next instruction. Otherwise, move backwards in the instructions to the corresponding '['
            b']' => vanilla::loop_close(tape, &pointer, &mut pc, code_bytes),

            // '\' sets the current cell to 10 (LFeed)
            b'\\' => if !vanilla { additional_inputs::line_feeed_input(tape, &pointer) },
            // b'c' sets the current cell to 'c' ASCII value
            b'b' => if !vanilla { additional_inputs::byte_input(tape, &pointer, &mut pc, code_bytes) },
            // s"abc" sets current cell to 'a' ASCII value, after that moves, and procedes to the next character (in this case 'b')
            b's' => if !vanilla { additional_inputs::string_input(tape, &mut pointer, &mut pc, &code_bytes) },
            // Numbers parsing (hex, dacimal or binary)
            b'0' => if !vanilla { additional_inputs::number_input(tape, &mut pointer, &mut pc, &code_bytes) },
            // Set current cell to 0
            b'^' => if !vanilla { additional_inputs::zero_input(tape, &pointer); },
            // Prints out cell content as digit
            b'p' => if !vanilla { additional_outputs::print_number(tape, &pointer); },

            b'f' => if !vanilla { file_operations::open_file(&mut pc, &code_bytes); },

            // Swaps current's and next cell's value
            b';' => if !vanilla { extras::swap(tape, &pointer); },

            // Comments
            b'/' => if !vanilla { extras::comment(&mut pc, &code_bytes); },
            _ => (),
        }
        pc += 1;
        if pc >= code.len() { break; }
    }
}

struct Operations {
    tape: [u8; 30000],
    code: String,
    vanilla: bool,
}

impl Operations {
    fn run(&mut self) {
        execute(self, Some(0), Some(0));
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

    let file_path = file_path.expect("Usage: BrainRust.exe [-v] <file_path>");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read file");

    let mut main = Operations {
        tape: [0; 30000],
        code: contents,
        vanilla: vanilla,
    };
    main.run();
    
}