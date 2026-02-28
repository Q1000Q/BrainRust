use crate::vanilla;

pub fn forward_extended(tape: &[u8], pointer: &mut usize, pc: &mut usize, code_bytes: &[u8]) {
    if !(code_bytes[*pc + 1] as char).is_digit(10) {
        vanilla::forward(tape, pointer);
        return;
    }

    let mut distance_str = String::new();
    while (code_bytes[*pc + 1] as char).is_digit(10) {
        *pc += 1;
        distance_str.push(code_bytes[*pc] as char);
    }

    let distance = usize::from_str_radix(&distance_str, 10).unwrap();

    *pointer = ((*pointer + distance) as isize).rem_euclid(tape.len() as isize) as usize;
}

pub fn backward_extended(tape: &[u8], pointer: &mut usize, pc: &mut usize, code_bytes: &[u8]) {
    if !(code_bytes[*pc + 1] as char).is_digit(10) {
        vanilla::backward(tape, pointer);
        return;
    }

    let mut distance_str = String::new();
    while (code_bytes[*pc + 1] as char).is_digit(10) {
        *pc += 1;
        distance_str.push(code_bytes[*pc] as char);
    }

    let distance = usize::from_str_radix(&distance_str, 10).unwrap();

    *pointer = ((*pointer - distance) as isize).rem_euclid(tape.len() as isize) as usize;
}