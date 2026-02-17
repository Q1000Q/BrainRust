pub fn swap(tape: &mut [u8], pointer: &usize) {
    tape[*pointer] = tape[*pointer] ^ tape[*pointer + 1];
    tape[*pointer + 1] = tape[*pointer] ^ tape[*pointer + 1];
    tape[*pointer] = tape[*pointer] ^ tape[*pointer + 1];
}

pub fn comment(pc: &mut usize, code_bytes: &[u8]) {
    if code_bytes[*pc + 1] != b'/' { return; }
    while *pc < code_bytes.len() && code_bytes[*pc] != b'\n' {
        *pc += 1;
    }
}