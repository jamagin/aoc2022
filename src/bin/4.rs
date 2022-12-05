use std::io;

use aoc2022::util::input_data_to_string;


struct Assignment {
    start: u64,
    end: u64,
}
struct AssignmentPair {
    first: Assignment,
    second: Assignment,
}

impl TryFrom<&str> for Assignment {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        let (start, end) = s.split_once('-').unwrap();
        Ok(Assignment {
            start: start.parse::<u64>().unwrap(),
            end: end.parse::<u64>().unwrap(),
        })
    }
}

impl TryFrom<&str> for AssignmentPair {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        let (first, second) = s.split_once(',').unwrap();
        Ok(AssignmentPair {
            first: Assignment::try_from(first).unwrap(),
            second: Assignment::try_from(second).unwrap(),
        })
    }
}


fn overlaps(a: &Assignment, b: &Assignment) -> bool {
    (a.start >= b.start) && (a.end <= b.end)
}

fn check(pair: AssignmentPair) -> bool {
    overlaps(&pair.first, &pair.second) || overlaps(&pair.second, &pair.first)
}

fn main() -> io::Result<()> {
    let mut count: u64 = 0;

    for input in input_data_to_string("4.txt")?.lines() {
        if check(AssignmentPair::try_from(input).unwrap()) {
            count += 1;
        }
        
    }

    println!("{}", count);
    Ok(())
}