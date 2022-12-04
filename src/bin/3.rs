use std::io;
use std::collections::HashSet;
use aoc2022::process_file::InputFile;

fn string_rep_to_priority_set(s: String) -> HashSet<u32> {
    HashSet::from_iter(s.chars().map(|c| char_to_priority(c)))
}

fn char_to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        panic!("invalid character: {}", c)
    }
}

fn main() -> io::Result<()> {
    let mut total_pt_1 = 0;
    let mut total_pt_2 = 0;
    let mut accumulate_three: Vec<HashSet<u32>> = Vec::new();
    let mut line_number = 0;

    for input in InputFile::new("3.txt") {
        let mut first_str = input;
        let second_str = first_str.split_off(first_str.len() / 2);
        let first_set = string_rep_to_priority_set(first_str);
        let second_set = string_rep_to_priority_set(second_str);
        let both_compartments = &first_set & &second_set;
        total_pt_1 += both_compartments.iter().sum::<u32>();
        accumulate_three.push(first_set);
        accumulate_three.push(second_set);
        line_number += 1;
        if line_number % 3 == 0 {
            println!("BATCH: {:#?}", accumulate_three);
            let mut tmp: HashSet<u32> = HashSet::new();
            for x in &accumulate_three {
                tmp = &tmp & &x;
            }
            total_pt_2 += tmp.iter().sum::<u32>();
            accumulate_three.clear();
        }
    };

    println!("{} {}", total_pt_1, total_pt_2);
    Ok(())
}
