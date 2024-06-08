use std::env;
use std::fs;
use std::io;

fn main() {
    // Reads BrainFuck code file from argument
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read file");
    // Creates tape and pointer
    let mut tape: [u8; 30000] = [0; 30000];
    let mut pointer: usize = 0;
    // Loops through whole code
    let mut pc: usize = 0;
    while pc < contents.len() {
        match contents.as_bytes()[pc] {
            // '>' adds 1 to pointer (moves pointer 1 right)
            b'>' => {
                pointer += 1;
                if pointer >= tape.len() {
                    pointer = 0;
                }
            },
            // '<' removes 1 to pointer (moves pointer 1 left)
            b'<' => {
                if pointer == 0 {
                    pointer = tape.len() - 1;
                } else {
                    pointer -= 1;
                }
            },
            // '+' adds 1 to localization at tape, that pointer points
            b'+' => tape[pointer] = tape[pointer].wrapping_add(1),
            // '-' removes 1 to localization at tape, that pointer points
            b'-' => tape[pointer] = tape[pointer].wrapping_sub(1),
            // '.' prints out content of tape cell that pointer is pointing (as a ASCII character)
            b'.' => print!("{}", tape[pointer] as char),
            // ',' require user to enter a ASCII character and saves it to the tape cell that pointer is pointing
            b',' => {
                let mut tmp = String::new();
                io::stdin().read_line(&mut tmp).expect("Failed to read line");
                if let Some(byte) = tmp.bytes().next() {
                    tape[pointer] = byte;
                }
            },
            // '[' starts loop, if the value at the current cell is 0, then skips to the corresponding ']'. Otherwise, move to the next instruction
            b'[' => {
                if tape[pointer] == 0 {
                    let mut loop_var: usize = 1;
                    while loop_var != 0 {
                        pc += 1;
                        if pc >= contents.len() {
                            panic!("Unmatched [");
                        }
                        match contents.as_bytes()[pc] {
                            b'[' => loop_var += 1,
                            b']' => loop_var -= 1,
                            _ => (),
                        }
                    }
                }
            },
            // ']' ends loop, f the value at the current cell is 0, move to the next instruction. Otherwise, move backwards in the instructions to the corresponding '['
            b']' => {
                if tape[pointer] != 0 {
                    let mut loop_var: usize = 1;
                    while loop_var != 0 {
                        if pc == 0 {
                            panic!("Unmatched ]");
                        }
                        pc -= 1;
                        match contents.as_bytes()[pc] {
                            b'[' => loop_var -= 1,
                            b']' => loop_var += 1,
                            _ => (),
                        }
                    }
                }
            },
            _ => (),
        }
        pc += 1;
    }
}