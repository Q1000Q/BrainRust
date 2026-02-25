use crate::vanilla;

pub fn line_feeed_input(tape: &mut [u8], pointer: &usize) {
    tape[*pointer] = 10;
}

pub fn byte_input(tape: &mut [u8], pointer: &usize, pc: &mut usize, code_bytes: &[u8]) {
    if code_bytes[*pc + 1] == b'\'' && code_bytes[*pc + 3] == b'\'' {
        tape[*pointer] = code_bytes[*pc + 2];
        *pc += 3;
    }
}

pub fn string_input(tape: &mut [u8], pointer: &mut usize, pc: &mut usize, code_bytes: &[u8]) {
    if code_bytes[*pc + 1] == b'\"' {
        *pc += 2;
        while code_bytes[*pc] != b'\"' {
            // Always saves character after '\'
            if code_bytes[*pc] == b'\\' {
                *pc += 1;
                tape[*pointer] = code_bytes[*pc];
            } else {
                tape[*pointer] = code_bytes[*pc];
            }

            vanilla::forward(tape, pointer);

            *pc += 1;
            if *pc >= code_bytes.len() { panic!("No ending double quote after command 's'") }
        }
    } else { panic!("Expected pait of double quotes after command 's'") }
}

pub fn number_input(tape: &mut [u8], pointer: &mut usize, pc: &mut usize, code_bytes: &[u8]) {
    *pc += 1;

    match code_bytes[*pc] {
        // Hex number: 0xAA inserts AA (170) into current cell (must have exacly 2 hex numbers after 0x)
        b'x' => {
            let hex_string: String = [code_bytes[*pc + 1] as char, code_bytes[*pc + 2] as char].iter().collect();
            *pc += 2;
            tape[*pointer] = u8::from_str_radix(&hex_string, 16).expect("Failed to parse hex string to int");
        }
        // Decimal number: 0d123 iserst 123 into current cell (must have exacly 3 decimal numbers after 0d)
        b'd' => {
            tape[*pointer] = str::from_utf8(&code_bytes[*pc + 1..*pc + 4])
                .unwrap()
                .parse::<u8>()
                .unwrap();
            *pc += 3;
        }
        // Binary number: 0d00110101 iserst 00110101 (53) into current cell (must have exacly 8 binary numbers after 0b)
        b'b' => {
            let binary_number = str::from_utf8(&code_bytes[*pc + 1..*pc + 9]).unwrap();
            *pc += 8;
            tape[*pointer] = u8::from_str_radix(binary_number, 2).expect("Failed to parse decimal string to int");
        }
        _ => { *pc -= 1 }
    }
}

pub fn zero_input(tape: &mut [u8], pointer: &usize) {
    tape[*pointer] = 0;
}

pub fn random_number_input(tape: &mut [u8], pointer: &usize) {
    let rnum: u8 = rand::random_range(0..255);
    tape[*pointer] = rnum;
}