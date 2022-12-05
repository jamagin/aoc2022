use std::fs::read_to_string;
use std::io::Result;
use std::path::Path;

pub fn input_data_to_string(file_in_data_dir: &str) -> Result<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("data").join(file_in_data_dir);
    read_to_string(path)
}