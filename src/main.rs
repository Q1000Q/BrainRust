use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read file");

    let mut tape: [u8; 30000] = [0; 30000];
    let mut pointer: usize = 0;
    
    let mut pc: usize = 0;
    while pc < contents.len() {
        match contents.as_bytes()[pc] {
            b'>' => {
                pointer += 1;
                if pointer >= tape.len() {
                    pointer = 0;
                }
            },
            b'<' => {
                if pointer == 0 {
                    pointer = tape.len() - 1;
                } else {
                    pointer -= 1;
                }
            },
            b'+' => tape[pointer] = tape[pointer].wrapping_add(1),
            b'-' => tape[pointer] = tape[pointer].wrapping_sub(1),
            b'.' => print!("{}", tape[pointer] as char),
            b',' => {
                let mut tmp = String::new();
                io::stdin().read_line(&mut tmp).expect("Failed to read line");
                if let Some(byte) = tmp.bytes().next() {
                    tape[pointer] = byte;
                }
            },
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