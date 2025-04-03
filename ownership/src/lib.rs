pub fn first_subword(mut s: String) -> String {
    let bytes = s.as_bytes();
    for i in 1..bytes.len() {
        let byte = bytes[i];
        if byte == b'_' || (byte as char).is_uppercase() {
            s.truncate(i);
            return s;
        }
    }
    s
}