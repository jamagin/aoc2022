use std::env;
use std::io;

use aoc2022::util::input_data_to_string;

// This approach feels really verbose but hopefully clear
#[derive(Copy, Clone)]
enum RPSMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone)]
enum RPSResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn wins(yours: RPSMove, mine: RPSMove) -> RPSResult {
    match yours {
        RPSMove::Rock => match mine {
            RPSMove::Rock => RPSResult::Draw,
            RPSMove::Paper => RPSResult::Win,
            RPSMove::Scissors => RPSResult::Loss,
        },
        RPSMove::Paper => match mine {
            RPSMove::Rock => RPSResult::Loss,
            RPSMove::Paper => RPSResult::Draw,
            RPSMove::Scissors => RPSResult::Win,
        },
        RPSMove::Scissors => match mine {
            RPSMove::Rock => RPSResult::Win,
            RPSMove::Paper => RPSResult::Loss,
            RPSMove::Scissors => RPSResult::Draw,
        },
    }
}

// I dislike the redundancy, wish I'd expressed this stuff in a way
// I could programatically invert it
fn get_move(yours: RPSMove, desired_result: RPSResult) -> RPSMove {
    match yours {
        RPSMove::Rock => match desired_result {
            RPSResult::Loss => RPSMove::Scissors,
            RPSResult::Draw => RPSMove::Rock,
            RPSResult::Win => RPSMove::Paper,
        },
        RPSMove::Paper => match desired_result {
            RPSResult::Loss => RPSMove::Rock,
            RPSResult::Draw => RPSMove::Paper,
            RPSResult::Win => RPSMove::Scissors,
        },
        RPSMove::Scissors => match desired_result {
            RPSResult::Loss => RPSMove::Paper,
            RPSResult::Draw => RPSMove::Scissors,
            RPSResult::Win => RPSMove::Rock,
        },
    }
}

fn play_part_1(yours: RPSMove, mine: RPSMove) -> u64 {
    wins(yours, mine) as u64 + mine as u64
}

fn play_part_2(yours: RPSMove, desired_result: RPSResult) -> u64 {
    let mine = get_move(yours, desired_result.clone());
    desired_result as u64 + mine as u64
}

fn parse_line_part_1(line: &str) -> (RPSMove, RPSMove) {
    let mut moves = line.split_whitespace();
    (
        match moves.next().expect("didn't get your move") {
            "A" => RPSMove::Rock,
            "B" => RPSMove::Paper,
            "C" => RPSMove::Scissors,
            x => panic!("invalid move: {}", x),
        },
        match moves.next().expect("didn't get my move") {
            "X" => RPSMove::Rock,
            "Y" => RPSMove::Paper,
            "Z" => RPSMove::Scissors,            
            x => panic!("invalid move: {}", x),
        },
    )
}

fn parse_line_part_2(line: &str) -> (RPSMove, RPSResult) {
    let mut moves = line.split_whitespace();
    (
        match moves.next().expect("didn't get your move") {
            "A" => RPSMove::Rock,
            "B" => RPSMove::Paper,
            "C" => RPSMove::Scissors,
            x => panic!("invalid move: {}", x),
        },
        match moves.next().expect("didn't get the result") {
            "X" => RPSResult::Loss,
            "Y" => RPSResult::Draw,
            "Z" => RPSResult::Win,            
            x => panic!("invalid result: {}", x),
        },
    )
}



fn main() -> io::Result<()> {
    let part = env::args().nth(1).expect("requires argument").parse::<u64>().expect("1 or 2");
    let mut total = 0;

    for input in input_data_to_string("2.txt")?.lines() {
        let score = match part {
            1 => {
                let (yours, mine) = parse_line_part_1(input);
                play_part_1(yours, mine)
            },
            2 => {
                let (yours, desired_result) = parse_line_part_2(input);
                play_part_2(yours, desired_result)

            },
            _ => panic!("invalid mode")
        };
        total += score;
    }
 
    println!("{}", total);
    Ok(())
}