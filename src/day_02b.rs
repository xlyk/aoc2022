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
        "A" => Ok(Move::Rock),
        "B" => Ok(Move::Paper),
        "C" => Ok(Move::Scissors),
        _ => Err("invalid input"),
    }
}

fn get_desired_outcome(input: &str) -> Result<Outcome, &str> {
    match input.to_uppercase().as_str() {
        "X" => Ok(Outcome::Lose),
        "Y" => Ok(Outcome::Draw),
        "Z" => Ok(Outcome::Win),
        _ => Err("invalid input"),
    }
}

fn get_required_move(them: Move, outcome: &Outcome) -> Move {
    match outcome {
        Outcome::Lose => match them {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        Outcome::Win => match them {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        Outcome::Draw => them
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

        let outcome = moves.next().unwrap();
        let outcome = &get_desired_outcome(outcome)?;

        let us = &get_required_move(them, outcome);
        let score = decide_score(us, outcome);

        println!("outcome: {:?}, score: {:?}", outcome, score);

        total_score += score;
    }

    println!("total_score: {:?}", total_score);

    Ok(())
}
