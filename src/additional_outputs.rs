pub fn print_number(tape: &[u8], pointer: &usize) {
    print!("{}", tape[*pointer]);
}

pub fn print_address(pointer: &usize) {
    print!("{}", pointer);
}