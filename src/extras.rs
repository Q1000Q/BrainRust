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

pub fn debug_dump(tape: &mut [u8], pointer: &usize) {
    let before = &tape[pointer.saturating_sub(4)..*pointer];
    let value = &tape[*pointer];
    let after = &tape[pointer.saturating_add(1).min(tape.len())..pointer.saturating_add(5).min(tape.len())];

    let before_string = before.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(" ");
    let after_string = after.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(" ");

    let all_before_string = format!("[{}] {}", pointer, before_string);
    println!("{} {} {}", all_before_string, value.to_string(), after_string);

    let spacers_count = all_before_string.len() + 1;
    let point_string = " ".repeat(spacers_count) + "^";
    println!("{}", point_string);
}