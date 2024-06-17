use core::panic;
use std::env;
use std::fs;
use std::io;

fn main() {
    // Reads BrainFuck code file from argument
    let args: Vec<String> = env::args().collect();
    let mut file_path: String = String::new();
    let mut vanilla: bool = false;
    for arg in args {
        match arg.as_str() {
            "-v" => vanilla = true, // Sets interpreter to work in vanilla mode, that means if command isn't in vanilla brainfuck it gonna skip it
            _ => file_path = arg,
        }
    }

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
            //
            // COMMANDS BELOW ARE NOT IN VANILLA BRAINFUCK, THEY WILL NOT EXECUTE WITH -v OPTION
            //
            // '\' sets current cell value to 10 (LFeed)
            b'\\' => {
                if vanilla {break;}
                tape[pointer] = 10},

            // 'b' checks if there is a pair of single quotes after it, if there is, saves ASCII character inside them as a value in current cell, otherwise program panics
            b'b' => {
                if vanilla {break;}
                if contents.as_bytes()[pc + 1] == b'\'' && contents.as_bytes()[pc + 3] == b'\'' {
                    tape[pointer] = contents.as_bytes()[pc + 2];
                    pc += 3;
                } else {
                    panic!("Expected pair of single quotes after keyword b");
                }
            },
            // 's' checks if there is a pair of double quotes after it, if there is, saves ASCII characters inside them, first as value in current cell, others in next ones
            b's' => {
                if vanilla {break;}
                if contents.as_bytes()[pc + 1] == b'\"' {   // Checks if there is double quote after s command
                    pc += 2;    // Adds 2 to pc to compensate for b and " characters before
                    while contents.as_bytes()[pc] != b'\"' {    // Loop through characters for as long as it isn't "
                        if contents.as_bytes()[pc] == b'\\' {
                            if contents.as_bytes()[pc + 1] == b'\"' {
                                pc += 1;
                                tape[pointer] = contents.as_bytes()[pc];    // Saves character after '\' ASCII table value to current cell
                            }
                        } else {
                            tape[pointer] = contents.as_bytes()[pc];    // Saves character ASCII table value to current cell                           
                        }
                        if contents.as_bytes()[pc + 1] != b'\"' {   // Checks if the next iteration will be last or not, if it's last, don't do next things
                            pointer += 1;
                            if pointer >= tape.len() {      // Checks if pointer exceeded tape length, reset pointer to 0 if yes
                                pointer = 0;
                            }
                        }
                        pc += 1;
                        if pc >= contents.len() {panic!("No ending double quote after command s")}
                    }
                } else {
                    panic!{"Expected pair of double quotes after command s"};
                }
            },
            // '0x' allows you to enter hex number, '0d' decimal and '0b' binary. These numbers will be entered in current cell
            b'0' => {
                if vanilla {break;} // Don't use if in vanilla mode
                pc += 1;
                match contents.as_bytes()[pc] { // Checks char after 0 to determinate if this is hex, decimal or binary number or just random 0
                    b'x' => {   // Behavior for hex number
                        let mut hex_str = String::new();    // Var for the hex number
                        loop {
                            pc += 1;
                            if contents.as_bytes()[pc].is_ascii_hexdigit() { // Checks if char after x and the next ones are hex characters
                                hex_str.push(contents.as_bytes()[pc] as char); // Adds hex char to hex_str var
                            } else {
                                pc -= 1;    // If it's not then moves position cursor one backwards and brakes the loop
                                break;
                            }
                        }
                        match u8::from_str_radix(&hex_str, 16) {    // Converts hex value into u8 and saves it to the current cell
                            Ok(number) => {
                                tape[pointer] = number;
                            },
                            Err(e) => {
                                println!("Failed to parse hex string to int: {}", e);
                            },
                        }
                    },
                    b'd' => { // Behavior for decimal number
                        let mut dec_str = String::new();
                        loop {
                            pc += 1;
                            if contents.as_bytes()[pc].is_ascii_digit() {   // Checks if char after x and the next ones are numbers
                                dec_str.push(contents.as_bytes()[pc] as char);  // Adds number to dec_str var
                            } else {
                                pc -= 1;    // If it's not then moves position cursor one backwards and brakes the loop
                                break;
                            }
                        }
                        match u8::from_str_radix(&dec_str, 10) {    // Converts decimal value into u8 and saves it to the current cell
                            Ok(number) => {
                                tape[pointer] = number;
                            },
                            Err(e) => {
                                println!("Failed to parse hex string to int: {}", e);
                            },
                        }
                    },
                    b'b' => { // Behavior for binary number
                        let mut bin_str = String::new();
                        loop {
                            pc += 1;
                            if contents.as_bytes()[pc] == b'0' || contents.as_bytes()[pc] == b'1' { // Checks if char after x and the next ones are 0 or 1
                                bin_str.push(contents.as_bytes()[pc] as char);  // Adds binary character to dec_str var
                            } else {
                                pc -= 1;    // If it's not then moves position cursor one backwards and brakes the loop
                                break;
                            }
                        }
                        // dec_str.parse::<u8>().unwrap();
                        match u8::from_str_radix(&bin_str, 2) {    // Converts binary value into u8 and saves it to the current cell
                            Ok(number) => {
                                tape[pointer] = number;
                            },
                            Err(e) => {
                                println!("Failed to parse hex string to int: {}", e);
                            },
                        }
                    },

                    _ => (),
                }
            },
            _ => (),
        }
        pc += 1;
    }
}