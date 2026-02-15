use std::io::stdin;

// Adds 1 to *pointer (moves *pointer 1 forward)
pub fn forward(tape: &[u8], pointer: &mut usize) {
    *pointer = if *pointer + 1 < tape.len() {
        *pointer + 1
    } else {
        0
    }
}

// Removes 1 from *pointer (moves *pointer 1 backward)
pub fn backwards(tape: &[u8], pointer: &mut usize) {
    *pointer = if *pointer == 0 {
        tape.len() - 1
    } else {
        *pointer - 1
    }
}

// Adds 1 to current cell
pub fn increment(tape: &mut [u8], pointer: &usize) {
    tape[*pointer] = tape[*pointer].wrapping_add(1);
}

// Removes 1 from current cell
pub fn decrement(tape: &mut [u8], pointer: &usize) {
    tape[*pointer] = tape[*pointer].wrapping_sub(1);
}

// Prints out content of tape cell that *pointer is pointing (as a ASCII character)
pub fn print(tape: &[u8], pointer: &usize) {
    print!("{}", tape[*pointer] as char);
}

// Require user to enter a ASCII character and saves it to the tape cell that *pointer is pointing
pub fn read(tape: &mut [u8], pointer: &usize) {
    let mut tmp = String::new();
    stdin().read_line(&mut tmp).expect("Failed to read line");
    if let Some(byte) = tmp.bytes().next() {
        tape[*pointer] = byte;
    }
}

// Starts loop, if the value at the current cell is 0, then skips to the corresponding ']'. Otherwise, move to the next instruction
pub fn loop_open(tape: &[u8], pointer: &usize, pc: &mut usize, code_bytes: &[u8]) {
    if tape[*pointer] == 0 {
        let mut loop_var: usize = 1;
        while loop_var != 0 {
            *pc += 1;
            if *pc >= code_bytes.len() {
                panic!("Unmatched [");
            }
            match code_bytes[*pc] {
                b'[' => loop_var += 1,
                b']' => loop_var -= 1,
                _ => (),
            }
        }
    }
}

// Ends loop, f the value at the current cell is 0, move to the next instruction. Otherwise, move backwards in the instructions to the corresponding '['
pub fn loop_close(tape: &[u8], pointer: &usize, pc: &mut usize, code_bytes: &[u8]) {
    if tape[*pointer] != 0 {
        let mut loop_var: usize = 1;
        while loop_var != 0 {
            if *pc == 0 {
                panic!("Unmatched ]");
            }
            *pc -= 1;
            match code_bytes[*pc] {
                b'[' => loop_var -= 1,
                b']' => loop_var += 1,
                _ => (),
            }
        }
    }
}