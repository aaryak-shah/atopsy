pub mod atop_raw_file;
pub mod constants;
pub mod utils;

fn main() {
    let file_path = "./atop.raw";
    atop_raw_file::parse_raw_file(file_path);
}
