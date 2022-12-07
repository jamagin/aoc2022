use std::io;
use std::collections::HashSet;

use aoc2022::util::input_data_to_string;

fn main() -> io::Result<()> {

    let input = input_data_to_string("6.txt")?;
    let chars: Vec<char> = input.chars().collect();
    let mut seen_sop = false;
    for i in (3+4)..chars.len() {
        let set_4: HashSet<char> = chars[i-4..i].iter().copied().collect();
        if !seen_sop && set_4.len() == 4 {
            seen_sop = true;
            println!("SOP {i}");
        }
        if i > 14 {
            let set_14: HashSet<char> = chars[i-14..i].iter().copied().collect();
            if set_14.len() == 14 {
                println!("SOM {i}");
                break;
            }
        }
    }
    Ok(())
}