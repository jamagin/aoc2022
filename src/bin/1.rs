use std::cmp;
use std::fs::File;
use std::io::{BufReader, self};
use std::io::prelude::*;
use std::path::Path;

fn main() -> io::Result<()> {
    let input_data = Path::new(env!("CARGO_MANIFEST_DIR")).join("data").join("1.txt");
    let fhandle = File::open(input_data)?;
    let br = BufReader::new(fhandle);
    let mut largest: u64 = 0;
    let mut current: u64 = 0;

    for line in br.lines() {
        let res = line.unwrap().parse::<u64>();
        match res {
            Err(_) => {
                largest = cmp::max(largest, current);
                current = 0;
            },
            Ok(item) => current += item,
        }
    }
    print!("{}", largest);
    Ok(())
}