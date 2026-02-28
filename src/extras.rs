use std::{io::{self, Write}, thread, time};

pub fn swap(tape: &mut [u8], pointer: &usize) {
    let next_pos = (*pointer + 1).rem_euclid(tape.len());
    tape[*pointer] = tape[*pointer] ^ tape[next_pos];
    tape[next_pos] = tape[*pointer] ^ tape[next_pos];
    tape[*pointer] = tape[*pointer] ^ tape[next_pos];
}

pub fn copy(tape: &mut [u8], pointer: &usize) {
    tape[(*pointer + 1).rem_euclid(tape.len())] = tape[*pointer];
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
    let before = &tape[pointer.saturating_sub(3)..*pointer];
    let value = &tape[*pointer];
    let after = &tape[pointer.saturating_add(1).min(tape.len())..pointer.saturating_add(4).min(tape.len())];

    let before_string = before.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(" ");
    let after_string = after.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(" ");

    let all_before_string = format!("[{}] {}", pointer, before_string);
    println!("\n{} {} {}", all_before_string, value.to_string(), after_string);

    let spacers_count = all_before_string.len() + 1;
    let point_string = " ".repeat(spacers_count) + "^";
    println!("{}", point_string);
}

// If current cell is anything other than 0, execute once
pub fn if_open(tape: &[u8], pointer: &usize, pc: &mut usize, code_bytes: &[u8]) {
    if tape[*pointer] == 0 {
        let mut if_var: usize = 1;
        while if_var != 0 {
            *pc += 1;
            if *pc >= code_bytes.len() {
                panic!("Unmatched (");
            }
            match code_bytes[*pc] {
                b'(' => if_var += 1,
                b')' => if_var -= 1,
                _ => (),
            }
        }
    }
}

pub fn access_relative_cell(pc: &mut usize, code_bytes: &[u8]) -> Option<(isize, String)> {
    if *pc + 1 >= code_bytes.len() {
        return None;
    }

    let rem = &code_bytes[(*pc + 1)..];
    let block_start = rem.iter().position(|&b| b == b'{')?;

    let distance_string: String = rem[..block_start]
        .iter()
        .map(|&b| b as char)
        .collect();

    if distance_string.is_empty() {
        return None;
    }
    
    let distance = isize::from_str_radix(&distance_string, 10).ok()?;

    let (code, block_len) = crate::parse_balanced_curly_block(rem, block_start)?;

    *pc += block_start + block_len;

    Some((distance, code))
}