use std::collections::HashMap;
use std::env;
use std::fs::{self};

mod vanilla;
mod additional_inputs;
mod additional_outputs;
mod file_operations;
mod extended;
mod extras;
mod macros;

fn execute(op: &mut Operations<'_>, pointer: Option<usize>, pc: Option<usize>) {
    let mut pointer: usize = pointer.unwrap_or(0);        
    let mut pc = pc.unwrap_or(0);   

    let vanilla = op.vanilla;
    let code_bytes = op.code.as_bytes().to_vec();
    let code_len = code_bytes.len();

    while pc < code_len {
        match code_bytes[pc] {
            // '>' adds 1 to pointer (moves pointer 1 forward)
            b'>' => if vanilla { vanilla::forward(op.tape, &mut pointer) } else { extended::forward_extended(op.tape, &mut pointer, &mut pc, &code_bytes); },
            // '<' removes 1 to pointer (moves pointer 1 backward)
            b'<' => if vanilla { vanilla::backward(op.tape, &mut pointer) } else { extended::backward_extended(op.tape, &mut pointer, &mut pc, &code_bytes); },
            // '+' adds 1 to localization at tape, that pointer points
            b'+' => vanilla::increment(op.tape, &pointer),
            // '-' removes 1 to localization at tape, that pointer points
            b'-' => vanilla::decrement(op.tape, &pointer),
            // '.' prints out content of tape cell that pointer is pointing (as a ASCII character)
            b'.' => vanilla::print(op.tape, &pointer),
            // ',' require user to enter a ASCII character and saves it to the tape cell that pointer is pointing
            b',' => vanilla::read(op.tape, &pointer),
            // '[' starts loop, if the value at the current cell is 0, then skips to the corresponding ']'. Otherwise, move to the next instruction
            b'[' => vanilla::loop_open(op.tape, &pointer, &mut pc, &code_bytes),
            // ']' ends loop, f the value at the current cell is 0, move to the next instruction. Otherwise, move backwards in the instructions to the corresponding '['
            b']' => vanilla::loop_close(op.tape, &pointer, &mut pc, &code_bytes),

            // '\' sets the current cell to 10 (LFeed)
            b'\\' => if !vanilla { additional_inputs::line_feeed_input(op.tape, &pointer) },
            // b'c' sets the current cell to 'c' ASCII value
            b'b' => if !vanilla { additional_inputs::byte_input(op.tape, &pointer, &mut pc, &code_bytes) },
            // s"abc" sets current cell to 'a' ASCII value, after that moves, and procedes to the next character (in this case 'b')
            b's' => if !vanilla { additional_inputs::string_input(op.tape, &mut pointer, &mut pc, &code_bytes) },
            // Numbers parsing (hex, dacimal or binary)
            b'0' => if !vanilla { additional_inputs::number_input(op.tape, &mut pointer, &mut pc, &code_bytes) },
            // Set current cell to 0
            b'^' => if !vanilla { additional_inputs::zero_input(op.tape, &pointer); },
            // Prints out cell content as digit
            b'p' => if !vanilla { additional_outputs::print_number(op.tape, &pointer); },
            // Prints out address of current cell (pointer value)
            b'A' => if !vanilla { additional_outputs::print_address(&pointer); },

            b'f' => if !vanilla { file_operations::open_file(&mut pc, &code_bytes); },
            b'r' => if !vanilla { file_operations::read_file(op.tape, &mut pointer, &mut pc, &code_bytes); },
            b'w' => if !vanilla { file_operations::write_tape_to_file(op.tape, &mut pc, &code_bytes); },
            b'a' => if !vanilla { file_operations::append_tape_to_file(op.tape, &mut pc, &code_bytes); },

            // Swaps current's and next cell's value
            b';' => if !vanilla { extras::swap(op.tape, &pointer); },

            // Comments
            b'/' => if !vanilla { extras::comment(&mut pc, &code_bytes); },


            // Define macro
            b'@' => if !vanilla {
                let mac = macros::define_macro(&mut pc, &code_bytes).unwrap();
                op.define_macro(mac);
            },
            // Get and run macro
            b'#' => if !vanilla {
                let macro_name = macros::get_macro(&mut pc, &code_bytes).unwrap();
                let macro_operations = op.get_macro_operations(macro_name).unwrap().clone();
                let macro_op = &mut Operations {
                    tape: &mut op.tape,
                    code: macro_operations,
                    vanilla: false,
                    macros: HashMap::new()
                };
                execute(macro_op, Some(pointer), Some(0));
            },
            _ => (),
        }
        pc += 1;
        if pc >= code_len { break; }
    }
}

struct Operations<'a> {
    tape: &'a mut [u8; 30000],
    code: String,
    vanilla: bool,
    macros: HashMap<String, String>,
}

impl<'a> Operations<'a> {
    fn run(&mut self) {
        execute(self, Some(0), Some(0));
    }

    fn define_macro(&mut self, mac: (String, String)) {
        self.macros.insert(mac.0, mac.1);
    }

    fn get_macro_operations(&self, macro_name: String) -> Option<&String> {
        return self.macros.get(&macro_name);
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

    let mut tape = [0; 30000];
    let mut main = Operations {
        tape: &mut tape,
        code: contents,
        vanilla: vanilla,
        macros: HashMap::new()
    };
    main.run();
    
}