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


fn full_overlap(a: &Assignment, b: &Assignment) -> bool {
    (a.start >= b.start) && (a.end <= b.end)
}

fn partial_overlap(a: &Assignment, b: &Assignment) -> bool {
    (a.end >= b.start) && (a.start <= b.end)
}

fn check_full(pair: &AssignmentPair) -> bool {
    full_overlap(&pair.first, &pair.second) || full_overlap(&pair.second, &pair.first)
}

fn check_partial(pair: &AssignmentPair) -> bool {
    partial_overlap(&pair.first, &pair.second) || partial_overlap(&pair.second, &pair.first)
}

fn main() -> io::Result<()> {
    let mut count_full: u64 = 0;
    let mut count_partial: u64 = 0;

    for input in input_data_to_string("4.txt")?.lines() {
        let pair = AssignmentPair::try_from(input).unwrap();
        if check_full(&pair) {
            count_full += 1;
        }
        if check_partial(&pair) {
            count_partial += 1;
        }
        
    }

    println!("{count_full} {count_partial}");
    Ok(())
}