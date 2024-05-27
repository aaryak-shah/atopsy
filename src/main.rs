pub mod constants;
pub mod rawfile;
pub mod utils;

use constants::{MAGIC, RAW_HEADER_SIZE};
use rawfile::RawHeader;
use std::fs;
use std::io::{BufReader, Read};

fn main() -> std::io::Result<()> {
    // read file in chunks
    let raw_file = fs::File::open("./atop.raw")?;
    let mut buf_reader = BufReader::new(raw_file);
    assert_eq!(
        std::mem::size_of::<RawHeader>(),
        RAW_HEADER_SIZE,
        "RawHeader struct size incorrect"
    );
    let mut contents = [0u8; RAW_HEADER_SIZE];
    buf_reader.read_exact(&mut contents)?;

    // extract magic number and compare with constant
    let mut magic_arr = [0u8; 4];
    magic_arr.clone_from_slice(&contents[0..4]);
    let magic = u32::from_be_bytes(magic_arr);

    if magic != MAGIC {
        panic!("FILE CORRUPTED")
    }

    Ok(())
}
