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
mod multi_tape;

fn execute(op: &mut Operations<'_>, pc: Option<usize>) {
    let mut pc = pc.unwrap_or(0);   
    
    let vanilla = op.vanilla;
    let code_bytes = op.code.as_bytes().to_vec();
    let code_len = code_bytes.len();
    let relative_file_path = op.relative_file_path.clone().unwrap_or_else(|| String::from("./"));
    
    let mut current_tape = *op.tapes.get(&op.current_tape_id).unwrap_or(&([0; 30000], 0));
    
    while pc < code_len {
        let tape = &mut current_tape.0;
        let pointer = &mut current_tape.1;
        match code_bytes[pc] {
            // '>' adds 1 to pointer (moves pointer 1 forward)
            b'>' => if vanilla { vanilla::forward(tape, pointer) } else { extended::forward_extended(tape, pointer, &mut pc, &code_bytes); },
            // '<' removes 1 to pointer (moves pointer 1 backward)
            b'<' => if vanilla { vanilla::backward(tape, pointer) } else { extended::backward_extended(tape, pointer, &mut pc, &code_bytes); },
            // '+' adds 1 to localization at tape, that pointer points
            b'+' => vanilla::increment(tape, pointer),
            // '-' removes 1 to localization at tape, that pointer points
            b'-' => vanilla::decrement(tape, pointer),
            // '.' prints out content of tape cell that pointer is pointing (as a ASCII character)
            b'.' => vanilla::print(tape, pointer),
            // ',' require user to enter a ASCII character and saves it to the tape cell that pointer is pointing
            b',' => vanilla::read(tape, pointer),
            // '[' starts loop, if the value at the current cell is 0, then skips to the corresponding ']'. Otherwise, move to the next instruction
            b'[' => vanilla::loop_open(tape, pointer, &mut pc, &code_bytes),
            // ']' ends loop, f the value at the current cell is 0, move to the next instruction. Otherwise, move backwards in the instructions to the corresponding '['
            b']' => vanilla::loop_close(tape, pointer, &mut pc, &code_bytes),

            // '\' sets the current cell to 10 (LFeed)
            b'\\' => if !vanilla { additional_inputs::line_feeed_input(tape, pointer) },
            // b'c' sets the current cell to 'c' ASCII value
            b'b' => if !vanilla { additional_inputs::byte_input(tape, pointer, &mut pc, &code_bytes) },
            // s"abc" sets current cell to 'a' ASCII value, after that moves, and procedes to the next character (in this case 'b')
            b's' => if !vanilla { additional_inputs::string_input(tape, pointer, &mut pc, &code_bytes) },
            // Numbers parsing (hex, dacimal or binary)
            b'0' => if !vanilla { additional_inputs::number_input(tape, pointer, &mut pc, &code_bytes) },
            // Set current cell to 0
            b'^' => if !vanilla { additional_inputs::zero_input(tape, pointer); },
            // Prints out cell content as digit
            b'p' => if !vanilla { additional_outputs::print_number(tape, pointer); },
            // Prints out address of current cell (pointer value)
            b'A' => if !vanilla { additional_outputs::print_address(pointer); },
            // Sets current cell value to random number
            b'R' => if !vanilla {additional_inputs::random_number_input(tape, pointer);},

            b'f' => if !vanilla { file_operations::open_file(&mut pc, &code_bytes, &relative_file_path); },
            b'r' => if !vanilla { file_operations::read_file(tape, pointer, &mut pc, &code_bytes, &relative_file_path); },
            b'w' => if !vanilla { file_operations::write_tape_to_file(tape, &mut pc, &code_bytes, &relative_file_path); },
            b'a' => if !vanilla { file_operations::append_tape_to_file(tape, &mut pc, &code_bytes, &relative_file_path); },

            // Swaps current and next cell value
            b';' => if !vanilla { extras::swap(tape, pointer); },
            // Copies current cell value to the next one
            b':' => if !vanilla { extras::copy(tape, pointer); },
            // Sleep for current cell value seconds
            b'S' => if !vanilla { extras::sleep(tape, pointer); }

            // Comments
            b'/' => if !vanilla { extras::comment(&mut pc, &code_bytes); },
            // Debug dump value with 3 before and after
            b'D' => if !vanilla { extras::debug_dump(tape, pointer); }


            // Define macro
            b'@' => if !vanilla {
                let mac = macros::define_macro(&mut pc, &code_bytes).unwrap();
                op.define_macro(mac);
            },
            // Run macro
            b'#' => if !vanilla {
                let macro_name = macros::get_macro(&mut pc, &code_bytes).unwrap();
                let macro_operations = op.get_macro_operations(macro_name).unwrap().clone();
                let macro_op = &mut Operations {
                    tapes: &mut op.tapes,
                    current_tape_id: *&op.current_tape_id,
                    code: macro_operations,
                    vanilla: false,
                    macros: HashMap::new(),
                    relative_file_path: Some(relative_file_path.clone())
                };
                execute(macro_op, Some(0));
            },

            // Change tape
            b'T' => if !vanilla { 
                op.tapes.insert(op.current_tape_id, current_tape);
                multi_tape::change_tape(&mut pc, &code_bytes, &mut op.tapes, &mut op.current_tape_id);
                current_tape = *op.tapes.get(&op.current_tape_id).unwrap_or(&([0; 30000], 0));
            }
            _ => (),
        }
        pc += 1;
        if pc >= code_len { break; }
    }
    
    // Save final tape state before returning
    op.tapes.insert(op.current_tape_id, current_tape);
}

struct Operations<'a> {
    tapes: &'a mut HashMap<u8, ([u8; 30000], usize)>,
    current_tape_id: u8,
    code: String,
    vanilla: bool,
    macros: HashMap<String, String>,
    relative_file_path: Option<String>,
}

impl<'a> Operations<'a> {
    fn run(&mut self) {
        execute(self, Some(0));
    }

    fn define_macro(&mut self, mac: (String, String)) {
        self.macros.insert(mac.0, mac.1);
    }

    fn get_macro_operations(&self, macro_name: String) -> Option<&String> {
        return self.macros.get(&macro_name);
    }
}

fn print_version() {
    let version: &str = env!("CARGO_PKG_VERSION");
    println!("v{}", version);
}

fn main() {
    // Reads Brainfuck code file from argument
    let args: Vec<String> = env::args().collect();
    let mut file_path: Option<String> = None;
    let mut vanilla: bool = false;
    for arg in &args[1..] {
        match arg.as_str() {
            "--vanilla" => vanilla = true,
            "-v" => print_version(),
            "--version" => print_version(),
            _ => file_path = Some(arg.to_string()),
        }
    }

    if file_path.is_none() {
        print!("Usage: brainrust [--vanilla] <file_path>");
        return;
    }

    let file_path = file_path.expect("Usage: brainrust [--vanilla] <file_path>");

    let contents = fs::read_to_string(&file_path)
        .expect("Should have been able to read file");

    let mut tapes = HashMap::new();
    tapes.insert(0, ([0; 30000], 0));

    let code_directory_path = Some(std::path::Path::new(&file_path).parent().unwrap_or(std::path::Path::new("./")).to_string_lossy().to_string() + "/");
    let mut main = Operations {
        tapes: &mut tapes,
        current_tape_id: b'0',
        code: contents,
        vanilla: vanilla,
        macros: HashMap::new(),
        relative_file_path: code_directory_path
    };
    main.run();
}