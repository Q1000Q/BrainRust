pub fn swap(tape: &mut [u8], pointer: &usize) {
    tape[*pointer] = tape[*pointer] ^ tape[*pointer + 1];
    tape[*pointer + 1] = tape[*pointer] ^ tape[*pointer + 1];
    tape[*pointer] = tape[*pointer] ^ tape[*pointer + 1];
}