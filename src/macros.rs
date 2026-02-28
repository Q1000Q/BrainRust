// Syntax: @(name){operations}
pub fn define_macro(pc: &mut usize, code_bytes: &[u8]) -> Option<(String, String)> {
    let rem = &code_bytes[(*pc + 1)..];

    if rem.first() != Some(&b'(') {
        return None;
    }

    let macro_name_end = rem.iter().position(|&b| b == b')')?;
    let macro_name = String::from_utf8_lossy(&rem[1..macro_name_end]).to_string();

    let block_start = macro_name_end + 1;
    let (operations, block_len) = crate::parse_balanced_curly_block(rem, block_start)?;

    *pc += block_start + block_len;

    Some((macro_name, operations))
}

pub fn get_macro(pc: &mut usize, code_bytes: &[u8]) -> Option<String> {
    let rem = &code_bytes[(*pc + 1)..];

    let mut iter = rem.iter();

    if iter.next() != Some(&b'(') { return None; }

    let macro_name: String = iter
        .take_while(|&b| b != &b')')
        .map(|&b| b as char)
        .collect();

    *pc += 1 + macro_name.len() + 1;

    return Some(macro_name);
}