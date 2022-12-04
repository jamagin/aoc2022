use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::path::Path;



pub struct InputFile {
    buf_reader_iter: Lines<BufReader<File>>,
}

impl InputFile {
    pub fn new(file_in_data_dir: &str) -> Self {
        let input_data = Path::new(env!("CARGO_MANIFEST_DIR")).join("data").join(file_in_data_dir);
        let fhandle = File::open(input_data).expect("failed to open input file");
        Self {
            buf_reader_iter: BufReader::new(fhandle).lines(),
        }
    }
}

impl Iterator for InputFile {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.buf_reader_iter.next() {
            Some(x) => Some(x.expect("failed to read line")),
            None => None,
        }
    }
}
