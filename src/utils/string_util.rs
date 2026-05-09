pub fn cut(origin: &mut String, cut_index: usize) {
    let mut str_bytes = vec![];
    let mut current_index: usize = 0;
    for byte in origin.bytes() {
        if current_index > cut_index {
            break;
        }
        str_bytes.push(byte);
    }
    *origin = String::from_utf8_lossy(&str_bytes).to_string();
}
