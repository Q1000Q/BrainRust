use std::{collections::HashMap, fs::{self, File}, io::{ErrorKind, Read, Write}};

use crate::Operations;

pub fn open_file(pc: &mut usize, code_bytes: &[u8], relative_file_path: &String) {
    let rem = &code_bytes[(*pc + 1)..];
    let mut iter = rem.iter();

    if iter.next() != Some(&b'(') { return; }

    let path_end = rem[1..].iter().position(|&b| b == b')').map(|idx| idx + 1);
    if path_end.is_none() { return; }
    let path_end = path_end.unwrap();

    let mut file_path = String::from_utf8_lossy(&rem[1..path_end]).to_string();

    *pc += path_end + 1;
    if file_path.len() == 0 { return; }


    let cwd_relative = file_path.bytes().nth(0).unwrap() == b'@';
    if cwd_relative { 
        file_path.remove(0);
    } else {
        file_path.insert_str(0, &relative_file_path);
    }

    let block_start = path_end + 1;
    let parsed_block = crate::parse_balanced_curly_block(rem, block_start);
    if parsed_block.is_none() { return; }
    let (file_code, block_len) = parsed_block.unwrap();
    *pc += block_len;

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

    // Read file content and put it to tapes
    let mut file_tapes: HashMap<u8, ([u8; 30000], usize)> = HashMap::new();
    if !created {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        for (tape_idx, chunk) in buf.chunks(30_000).enumerate().take(207) {
            let mut tape_array = [0u8; 30000];
            tape_array[..chunk.len()].copy_from_slice(chunk);
            file_tapes.insert((tape_idx + 48) as u8, (tape_array, 0));
        }
    }
    if file_tapes.get(&b'0').is_none() {
        file_tapes.insert(b'0', ([0; 30000], 0));
    }

    // Run operations on file
    let mut file_operations = Operations {
        tapes: &mut file_tapes,
        current_tape_id: 48,
        code: file_code,
        vanilla: false,
        macros: HashMap::new(),
        relative_file_path: None
    };
    file_operations.run();

    // Get tapes without pointers
    let mut file_tapes: HashMap<u8, Vec<u8>> = file_operations.tapes
        .iter()
        .filter(|(_, (tape, _))| tape.iter().any(|&b| b != 0))
        .map(|(k, (tape, _))| (*k, tape.to_vec()))
        .collect();

    // Remove zeroes from end of the last tape
    if let Some((&max_key, last_tape)) = file_tapes.iter().max_by_key(|(k, _)| **k) {
        let mut trimmed_tape = last_tape.clone();
        while trimmed_tape.last() == Some(&0) {
            trimmed_tape.pop();
        }
        file_tapes.insert(max_key, trimmed_tape);
    }

    // Write new file content
    let mut sorted_tapes: Vec<_> = file_tapes.iter().collect();
    sorted_tapes.sort_by_key(|(k, _)| *k);
    let buf: Vec<u8> = sorted_tapes.iter().flat_map(|(_, v)| v.iter().copied()).collect();
    if created {
        file.write_all(&buf).unwrap();
    } else {
        fs::remove_file(&file_path).unwrap();
        File::create(&file_path).unwrap().write_all(&buf).unwrap();
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
        *pointer = (*pointer + 1).rem_euclid(tape.len());
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