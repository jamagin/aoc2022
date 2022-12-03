use std::fs::File;
use std::io::{BufReader, self};
use std::io::prelude::*;
use std::path::Path;

// This approach feels really verbose but hopefully clear
#[derive(Copy, Clone)]
enum RPSMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

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

fn play(yours: RPSMove, mine: RPSMove) -> u64 {
    wins(yours, mine) as u64 + mine as u64
}

fn parse_line(line: String) -> (RPSMove, RPSMove) {
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

fn main() -> io::Result<()> {
    let input_data = Path::new(env!("CARGO_MANIFEST_DIR")).join("data").join("2.txt");
    let fhandle = File::open(input_data)?;
    let br = BufReader::new(fhandle);
    let mut total = 0;

    for line in br.lines() {
        let input = line.expect("could not read line");
        let (yours, mine) = parse_line(input);
        let score = play(yours, mine);
        total += score;
    }
 
    println!("{}", total);
    Ok(())
}