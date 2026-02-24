use std::collections::HashMap;

pub fn change_tape(pc: &mut usize, code_bytes: &[u8], tapes: &mut HashMap<u8, ([u8; 30000], usize)>, current_tape_id: &mut u8) {
    if *pc + 1 >= code_bytes.len() { return; }
    *pc += 1;
    let new_tape = code_bytes[*pc];

    *current_tape_id = new_tape;
    if tapes.get(*&current_tape_id).is_none() {
        tapes.insert(*current_tape_id, ([0; 30000], 0));
    }
}