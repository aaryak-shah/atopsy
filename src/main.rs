pub mod atop_raw_file;
pub mod constants;
pub mod rules;
pub mod types;
pub mod utils;

fn main() {
    let file_path = "./atop-new.raw";
    atop_raw_file::parse_raw_file(file_path);
}
