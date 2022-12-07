use std::io;
use std::collections::HashSet;

use aoc2022::util::input_data_to_string;

fn main() -> io::Result<()> {

    let input = input_data_to_string("6.txt")?;
    let chars: Vec<char> = input.chars().collect();
    for i in (3+4)..chars.len() {
        let set: HashSet<char> = chars[i-4..i].iter().copied().collect();
        if set.len() == 4 {
            println!("{i}");
            break;
        }
    }
    Ok(())
}