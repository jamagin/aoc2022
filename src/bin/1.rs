use std::collections::BinaryHeap;
use std::io;

use aoc2022::process_file::process_lines;

fn main() -> io::Result<()> {
    let mut totals: BinaryHeap<u64> = BinaryHeap::new();
    let mut current: u64 = 0;

    process_lines("1.txt", |input| {
        let res = input.parse::<u64>();
        match res {
            Err(_) => {
                totals.push(current);
                current = 0;
            },
            Ok(item) => current += item,
        }
    })?;
    totals.push(current);

    let top = totals.into_sorted_vec();
    println!("{}", top[top.len() - 3 ..].iter().sum::<u64>());
    Ok(())
}