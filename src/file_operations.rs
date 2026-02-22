use std::{collections::HashMap, fs::{self, File}, io::{ErrorKind, Read, Write}};

use crate::Operations;

pub fn open_file(pc: &mut usize, code_bytes: &[u8], relative_file_path: &String) {
    let rem = &code_bytes[(*pc + 1)..];
    let mut iter = rem.iter();

    if iter.next() != Some(&b'(') { return; }
    let mut file_path: String = iter
        .by_ref()
        .take_while(|&b| b != &b')')
        .map(|&b| b as char)
        .collect();

    *pc += 1 + file_path.len() + 1;
    if file_path.len() == 0 { return; }


    let cwd_relative = file_path.bytes().nth(0).unwrap() == b'@';
    if cwd_relative { 
        file_path.remove(0);
    } else {
        file_path.insert_str(0, &relative_file_path);
    }

    if iter.next() != Some(&b'{') { return; }

    let mut created: bool = false;
    let mut file = File::options().read(true).write(true).open(&file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            created = true;
            File::create(&file_path).unwrap_or_else(|error|{
                panic!("Problem creating file: {}", error);
            })
        } else {
            panic!("Problem opening file: {}", error);
        }
    });

    // Read file content and put it to tape
    let mut file_tape: [u8; 30000] = [0; 30000];
    if !created {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        for (i, &byte) in buf.iter().enumerate().take(30000) {
            file_tape[i] = byte;
        }
    }

    // Save code to execute on file
    let file_code: String = iter
        .take_while(|&b| b != &b'}')
        .map(|&b| b as char)
        .collect();

    *pc += 1 + file_code.len() + 1;

    // Run operations on file
    let mut file_operations = Operations {
        tape: &mut file_tape,
        code: file_code,
        vanilla: false,
        macros: HashMap::new(),
        relative_file_path: None
    };
    file_operations.run();
    let file_tape = file_operations.tape;

    // Remove zeroes from end of the tape
    let mut trimmed_tape: Vec<u8> = file_tape.iter().copied().collect();
    while trimmed_tape.last() == Some(&0) {
        trimmed_tape.pop();
    }

    // Write new file content
    let buf: &[u8] = &trimmed_tape;
    if created {
        file.write_all(buf).unwrap();
    } else {
        fs::remove_file(&file_path).unwrap();
        File::create(&file_path).unwrap().write_all(buf).unwrap();
    }
}

// Read file content to tape at the current cell and number of next ones
pub fn read_file(tape: &mut [u8], pointer: &mut usize, pc: &mut usize, code_bytes: &[u8], relative_file_path: &String) {
    let rem = &code_bytes[(*pc + 1)..];
    let mut iter = rem.iter();

    if iter.next() != Some(&b'(') { return; }
    let mut file_path: String = iter
        .by_ref()
        .take_while(|&b| b != &b')')
        .map(|&b| b as char)
        .collect();

    if file_path.len() == 0 { return; }
    *pc += 1 + file_path.len() + 1;

    let cwd_relative = file_path.bytes().nth(0).unwrap() == b'@';
    if cwd_relative { 
        file_path.remove(0);
    } else {
        file_path.insert_str(0, &relative_file_path);
    }

    if file_path.len() == 0 { return; }

    let file_result = File::options().read(true).write(false).open(&file_path);
    let mut file = match file_result {
        Ok(f) => f,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                return;
            } else {
                panic!("Problem opening file: {}", error);
            }
        }
    };

    let mut file_buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_buf).expect("Problem with reading the file");
    file_buf.truncate(tape.len());

    for &ch in file_buf.iter() {
        tape[*pointer] = ch;
        *pointer = (*pointer + 1) % tape.len();
    }
}

// Write tape content to file (from cell 0)
pub fn write_tape_to_file(tape: &mut [u8], pc: &mut usize, code_bytes: &[u8], relative_file_path: &String) {
    let rem = &code_bytes[(*pc + 1)..];
    let mut iter = rem.iter();

    if iter.next() != Some(&b'(') { return; }
    let mut file_path: String = iter
        .by_ref()
        .take_while(|&b| b != &b')')
        .map(|&b| b as char)
        .collect();

    if file_path.len() == 0 { return; }
    *pc += 1 + file_path.len() + 1;

    let cwd_relative = file_path.bytes().nth(0).unwrap() == b'@';
    if cwd_relative { 
        file_path.remove(0);
    } else {
        file_path.insert_str(0, &relative_file_path);
    }

    if file_path.len() == 0 { return; }

    let mut created: bool = false;
    let mut file = File::options().read(true).write(true).open(&file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            created = true;
            File::create(&file_path).unwrap_or_else(|error|{
                panic!("Problem creating file: {}", error);
            })
        } else {
            panic!("Problem opening file: {}", error);
        }
    });

    let mut trimmed_tape: Vec<u8> = tape.iter().copied().collect();
    while trimmed_tape.last() == Some(&0) {
        trimmed_tape.pop();
    }

    if created {
        file.write_all(&*trimmed_tape).unwrap();
    } else {
        fs::remove_file(&file_path).unwrap();
        File::create(&file_path).unwrap().write_all(&*trimmed_tape).unwrap();
    }
}

// Append tape content to file (from cell 0)
pub fn append_tape_to_file(tape: &mut [u8], pc: &mut usize, code_bytes: &[u8], relative_file_path: &String) {
    let rem = &code_bytes[(*pc + 1)..];
    let mut iter = rem.iter();

    if iter.next() != Some(&b'(') { return; }
    let mut file_path: String = iter
        .by_ref()
        .take_while(|&b| b != &b')')
        .map(|&b| b as char)
        .collect();

    if file_path.len() == 0 { return; }
    *pc += 1 + file_path.len() + 1;

    let cwd_relative = file_path.bytes().nth(0).unwrap() == b'@';
    if cwd_relative { 
        file_path.remove(0);
    } else {
        file_path.insert_str(0, &relative_file_path);
    }

    if file_path.len() == 0 { return; }

    let mut file = File::options().read(true).append(true).open(&file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&file_path).unwrap_or_else(|error|{
                panic!("Problem creating file: {}", error);
            })
        } else {
            panic!("Problem opening file: {}", error);
        }
    });

    let mut trimmed_tape: Vec<u8> = tape.iter().copied().collect();
    while trimmed_tape.last() == Some(&0) {
        trimmed_tape.pop();
    }

    file.write_all(&*trimmed_tape).unwrap();
}