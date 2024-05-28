pub fn string_from_bytes(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap_or(String::from(""))
}
