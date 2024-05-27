pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let mut arr = [0u8; 4];
    arr.clone_from_slice(&bytes[0..4]);
    u32::from_be_bytes(arr)
}

pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    let mut arr = [0u8; 2];
    arr.clone_from_slice(&bytes[0..2]);
    u16::from_be_bytes(arr)
}

pub fn bytes_to_i32(bytes: &[u8]) -> i32 {
    let mut arr = [0u8; 4];
    arr.clone_from_slice(&bytes[0..4]);
    i32::from_be_bytes(arr)
}

pub fn bytes_to_i16(bytes: &[u8]) -> i16 {
    let mut arr = [0u8; 2];
    arr.clone_from_slice(&bytes[0..2]);
    i16::from_be_bytes(arr)
}
