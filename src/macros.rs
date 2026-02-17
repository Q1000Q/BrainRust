// Syntax: @(name){operations}
pub fn define_macro(pc: &mut usize, code_bytes: &[u8]) -> Option<(String, String)> {
    let rem = &code_bytes[(*pc + 1)..];

    let mut iter = rem.iter();

    if iter.next() != Some(&b'(') { return None; }

    let macro_name: String = iter
        .by_ref()
        .take_while(|&b| b != &b')')
        .map(|&b| b as char)
        .collect();

    if iter.next() != Some(&b'{') { return None; }
    
    let operations: String = iter
        .take_while(|&b| b != &b'}')
        .map(|&b| b as char)
        .collect();

    *pc += 1 + macro_name.len() + 1 + 1 + operations.len() + 1;

    return Some((macro_name, operations));
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