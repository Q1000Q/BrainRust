pub fn swap(tape: &mut [u8], pointer: &usize) {
    tape[*pointer] = tape[*pointer] ^ tape[*pointer + 1];
    tape[*pointer + 1] = tape[*pointer] ^ tape[*pointer + 1];
    tape[*pointer] = tape[*pointer] ^ tape[*pointer + 1];
}

pub fn comment(pc: &mut usize, code_bytes: &[u8]) {
    while *pc < code_bytes.len() && code_bytes[*pc] as char != '\n' {
        *pc += 1;
    }
}