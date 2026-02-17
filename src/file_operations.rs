use std::{fs::{self, File}, io::{ErrorKind, Read, Write}};

use crate::Operations;

pub fn open_file(pc: &mut usize, code_bytes: &[u8]) {
    if code_bytes[*pc + 1] != b'(' { return; }
    *pc += 2;
    let mut file_path = String::new();
    while code_bytes[*pc] != b')' {
        file_path.push(code_bytes[*pc] as char);
        *pc += 1;
    }

    if code_bytes[*pc + 1] != b'{' { return; }
    *pc += 1;

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
    *pc += 1;
    let mut file_code = String::new();
    while code_bytes[*pc] != b'}' {
        file_code.push(code_bytes[*pc] as char);
        *pc += 1;
    }

    let mut file_operations = Operations {
        tape: file_tape,
        code: file_code,
        vanilla: false
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
