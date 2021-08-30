pub fn string_to_char(s: &str) -> Option<char> {
    let mut chars = s.chars();
    let radical = chars.next();
    match chars.next() {
        Some(_) => None,
        None => radical,
    }
}