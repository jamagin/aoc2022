use std::fs::File;
use std::io::{BufReader, self};
use std::io::prelude::*;
use std::ops::FnMut;
use std::path::Path;

pub fn process_lines<F>(file_in_data_dir: &str, mut code: F) -> io::Result<()>
where
    F: FnMut(String),
{
    let input_data = Path::new(env!("CARGO_MANIFEST_DIR")).join("data").join(file_in_data_dir);
    let fhandle = File::open(input_data)?;
    let br = BufReader::new(fhandle);
    for line in br.lines() {
        let input = line.expect("could not read line");
        code(input);
    }
    Ok(())
}