use std::io;
use std::collections::HashSet;
use aoc2022::process_file::process_lines;

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
    let mut total = 0;

    process_lines("3.txt", |input| {
        let mut first_str = input;
        let second_str = first_str.split_off(first_str.len() / 2);
        let first_set = string_rep_to_priority_set(first_str);
        let second_set = string_rep_to_priority_set(second_str);
        total += first_set.intersection(&second_set).sum::<u32>();
    })?;

    println!("{}", total);
    Ok(())
}
