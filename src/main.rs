use core::panic;
use std::env;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;

mod vanilla;

struct Operations {
    tape: [u8; 30000],
    pointer: usize,
    code: String,
    pc: usize,
    vanilla: bool,
}
impl Operations {
    fn execute(&mut self) {
        let mut tape = self.tape;
        let mut pointer = self.pointer;
        let code = &self.code;
        let mut pc = self.pc;
        let vanilla = self.vanilla;
        
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
                
                // COMMANDS BELOW ARE NOT IN VANILLA BRAINFUCK, THEY WILL NOT EXECUTE WITH -v OPTION
                
                // '\' sets current cell value to 10 (LFeed)
                b'\\' => {
                    if vanilla {break;}
                    tape[pointer] = 10},
    
                // 'b' checks if there is a pair of single quotes after it, if there is, saves ASCII character inside them as a value in current cell, otherwise program panics
                b'b' => {
                    if vanilla {break;}
                    if code_bytes[pc + 1] == b'\'' && code_bytes[pc + 3] == b'\'' {
                        tape[pointer] = code_bytes[pc + 2];
                        pc += 3;
                    } else {
                        panic!("Expected pair of single quotes after keyword b");
                    }
                },
                // 's' checks if there is a pair of double quotes after it, if there is, saves ASCII characters inside them, first as value in current cell, others in next ones
                b's' => {
                    if vanilla {break;}
                    if code_bytes[pc + 1] == b'\"' {   // Checks if there is double quote after s command
                        pc += 2;    // Adds 2 to pc to compensate for b and " characters before
                        while code_bytes[pc] != b'\"' {    // Loop through characters for as long as it isn't "
                            if code_bytes[pc] == b'\\' {
                                if code_bytes[pc + 1] == b'\"' {
                                    pc += 1;
                                    tape[pointer] = code_bytes[pc];    // Saves character after '\' ASCII table value to current cell
                                }
                            } else {
                                tape[pointer] = code_bytes[pc];    // Saves character ASCII table value to current cell                           
                            }
                            if code_bytes[pc + 1] != b'\"' {   // Checks if the next iteration will be last or not, if it's last, don't do next things
                                pointer += 1;
                                if pointer >= tape.len() {      // Checks if pointer exceeded tape length, reset pointer to 0 if yes
                                    pointer = 0;
                                }
                            }
                            pc += 1;
                            if pc >= code.len() {panic!("No ending double quote after command s")}
                        }
                    } else {
                        panic!{"Expected pair of double quotes after command s"};
                    }
                },
                // '0x' allows you to enter hex number, '0d' decimal and '0b' binary. These numbers will be entered in current cell
                b'0' => {
                    if vanilla {break;} // Don't use if in vanilla mode
                    pc += 1;
                    match code_bytes[pc] { // Checks char after 0 to determinate if this is hex, decimal or binary number or just random 0
                        b'x' => {   // Behavior for hex number
                            let mut hex_str = String::new();    // Var for the hex number
                            loop {
                                pc += 1;
                                if pc >= code.len() {break;}
                                if code_bytes[pc].is_ascii_hexdigit() { // Checks if char after x and the next ones are hex characters
                                    hex_str.push(code_bytes[pc] as char); // Adds hex char to hex_str var
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
                                if pc >= code.len() {break;}
                                if code_bytes[pc].is_ascii_digit() {   // Checks if char after x and the next ones are numbers
                                    dec_str.push(code_bytes[pc] as char);  // Adds number to dec_str var
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
                                    println!("Failed to parse decimal string to int: {}", e);
                                },
                            }
                        },
                        b'b' => { // Behavior for binary number
                            let mut bin_str = String::new();
                            loop {
                                pc += 1;
                                if pc >= code.len() {break;}
                                if code_bytes[pc] == b'0' || code_bytes[pc] == b'1' { // Checks if char after x and the next ones are 0 or 1
                                    bin_str.push(code_bytes[pc] as char);  // Adds binary character to dec_str var
                                } else {
                                    pc -= 1;    // If it's not then moves position cursor one backwards and brakes the loop
                                    break;
                                }
                            }
                            match u8::from_str_radix(&bin_str, 2) {    // Converts binary value into u8 and saves it to the current cell
                                Ok(number) => {
                                    tape[pointer] = number;
                                },
                                Err(e) => {
                                    println!("Failed to parse binary string to int: {}", e);
                                },
                            }
                        },
                        _ => (),
                    }
                },
                b'f' => {
                    if vanilla {break;}
                    if code_bytes[pc + 1] != b'[' {break;} // Checks if after f it is [ character, if not brake
                    pc += 2;
                    let mut path_tmp = String::new();
                    while code_bytes[pc] != b']' {
                        path_tmp.push(code_bytes[pc] as char); 
                        pc += 1;
                    }
                    if code_bytes[pc + 1] != b'{' {break;} // Checks if there is { character, if not brake
                    pc += 1;

                    let mut created: bool = false; // Variable that will be false if file didn't exist before and true if it did
                    let mut operation_file = File::options().read(true).write(true).open(&path_tmp).unwrap_or_else(|error| { // Opens file if it exists, if not creates one
                        if error.kind() == ErrorKind::NotFound {
                            created = true;
                            File::create(&path_tmp).unwrap_or_else(|error| {
                                panic!("Problem creating file: {}", error);
                            })
                        } else {
                            panic!("Problem opening file: {}", error);
                        }
                    });

                    // Reads file content and puts it to tape
                    let mut tape_tmp: [u8; 30000] = [0; 30000]; // Fills tape_tmp with 0
                    if !created {
                        let mut buf = Vec::new();    // Create buffer Vec
                        operation_file.read_to_end(&mut buf).unwrap();    // Reads file content to buffer
                        for (i, &byte) in buf.iter().enumerate().take(30000) {  // Iterate over first 30000 characters of file, enumerate
                            tape_tmp[i] = byte; // Inserts first 30000 characters of file into tape_tmp
                        }
                    }
                    
                    // Check for end of operations
                    pc += 1;
                    let mut code_tmp = String::new();
                    while code_bytes[pc] != b'}' {
                        code_tmp.push(code_bytes[pc] as char);
                        pc += 1;
                    }

                    // Creates new Operations struct for file
                    let mut operate_file = Operations {
                        tape: tape_tmp,
                        pointer: 0,
                        code: code_tmp,
                        pc: 0,
                        vanilla: vanilla,
                    };
                    operate_file.execute();
                    let tape_tmp = operate_file.tape;

                    // Removes 0s (NULLs) from the end of the tape
                    let mut new_file: Vec<u8> = Vec::new();
                    for &char in tape_tmp.iter().rev() {
                        if char == 0 {
                            continue;
                        }
                        new_file.push(char);
                    }
                    new_file.reverse();

                    // Writes new tape to file
                    let buf: &[u8] = &new_file;
                    if created {
                        operation_file.write_all(buf).unwrap();
                    } else {
                        fs::remove_file(&path_tmp).unwrap();
                        File::create(&path_tmp).unwrap().write_all(buf).unwrap();
                    }
                },
                _ => (),
            }
            pc += 1;
            if pc >= code.len() {break;}
        };
        self.tape = tape;
        self.pointer = pointer;
        self.pc = pc;
    }
}
fn main() {
    // Reads BrainFuck code file from argument
    let args: Vec<String> = env::args().collect();
    let mut file_path: Option<String> = None;
    let mut vanilla: bool = false;
    for arg in args {
        match arg.as_str() {
            "-v" => vanilla = true, // Sets interpreter to work in vanilla mode, that means if command isn't in vanilla brainfuck it gonna skip it
            _ => file_path = Some(arg),
        }
    }

    let file_path = file_path.expect("Usage: BrainRust.exe [-v] <file_path>");

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