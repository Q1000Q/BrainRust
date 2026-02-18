use std::{collections::HashMap, fs::{self, File}, io::{ErrorKind, Read, Write}};

use crate::Operations;

pub fn open_file(pc: &mut usize, code_bytes: &[u8]) {
    let rem = &code_bytes[(*pc + 1)..];
    let mut iter = rem.iter();

    if iter.next() != Some(&b'(') { return; }
    let file_path: String = iter
        .by_ref()
        .take_while(|&b| b != &b')')
        .map(|&b| b as char)
        .collect();

    if file_path.len() == 0 { return; }

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

    *pc += 1 + file_path.len() + 1 + 1 + file_code.len() + 1;

    // Run operations on file
    let mut file_operations = Operations {
        tape: &mut file_tape,
        code: file_code,
        vanilla: false,
        macros: HashMap::new()
    };
    file_operations.run();
    let file_tape = file_operations.tape;

    // Remove zeroes from end of the tape
    let mut new_file_content: Vec<u8> = Vec::new();
    for &ch in file_tape.iter().rev() {
        if ch == 0 {
            continue;
        }
        new_file_content.push(ch);
    }
    new_file_content.reverse();

    // Write new file content
    let buf: &[u8] = &new_file_content;
    if created {
        file.write_all(buf).unwrap();
    } else {
        fs::remove_file(&file_path).unwrap();
        File::create(&file_path).unwrap().write_all(buf).unwrap();
    }
}
