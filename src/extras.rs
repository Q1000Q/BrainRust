use std::{io::{self, Write}, thread, time};

pub fn swap(tape: &mut [u8], pointer: &usize) {
    tape[*pointer] = tape[*pointer] ^ tape[(*pointer + 1) % tape.len()];
    tape[(*pointer + 1) % tape.len()] = tape[*pointer] ^ tape[(*pointer + 1) % tape.len()];
    tape[*pointer] = tape[*pointer] ^ tape[(*pointer + 1) % tape.len()];
}

pub fn copy(tape: &mut [u8], pointer: &usize) {
    tape[(*pointer + 1) % tape.len()] = tape[*pointer];
}

pub fn comment(pc: &mut usize, code_bytes: &[u8]) {
    if *pc + 1 >= code_bytes.len() || code_bytes[*pc + 1] != b'/' { return; }
    while *pc < code_bytes.len() && code_bytes[*pc] != b'\n' {
        *pc += 1;
    }
}

pub fn sleep(tape: &mut [u8], pointer: &usize) {
    io::stdout().flush().unwrap();
    let time = time::Duration::from_secs(tape[*pointer] as u64);
    thread::sleep(time);
}