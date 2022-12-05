use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Lose,
    Win,
    Draw,
}

fn get_move(input: &str) -> Result<Move, &str> {
    match input.to_uppercase().as_str() {
        "A" | "X" => Ok(Move::Rock),
        "B" | "Y" => Ok(Move::Paper),
        "C" | "Z" => Ok(Move::Scissors),
        _ => Err("invalid input"),
    }
}

fn decide_win(us: &Move, them: Move) -> Outcome {
    match us {
        Move::Rock => {
            match them {
                Move::Scissors => Outcome::Win,
                Move::Paper => Outcome::Lose,
                Move::Rock => Outcome::Draw
            }
        }
        Move::Paper => {
            match them {
                Move::Rock => Outcome::Win,
                Move::Scissors => Outcome::Lose,
                Move::Paper => Outcome::Draw
            }
        }
        Move::Scissors => {
            match them {
                Move::Paper => Outcome::Win,
                Move::Rock => Outcome::Lose,
                Move::Scissors => Outcome::Draw
            }
        }
    }
}

fn decide_score(us: &Move, outcome: &Outcome) -> i32 {
    let a = match us {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    let b = match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };

    a + b
}

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut total_score = 0;

    let file = File::open("input_2a.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut moves = line.split_whitespace();
        let them = get_move(moves.next().unwrap())?;
        let us = &get_move(moves.next().unwrap())?;
        let outcome = &decide_win(us, them);
        let score = decide_score(us, outcome);

        println!("outcome: {:?}, score: {:?}", outcome, score);

        total_score += score;
    }

    println!("total_score: {:?}", total_score);

    Ok(())
}
