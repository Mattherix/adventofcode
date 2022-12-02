use std::fs::File;
use std::io::{Read, self};
use std::path::Path;

#[derive(Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors
}

// Can panic (parse)
fn extract<P: AsRef<Path>>(path: P) -> Result<Vec<(Move, Move)>, io::Error> {
    let mut input = File::open(path)?;

    let mut s = String::new();
    input.read_to_string(&mut s)?;

    let lines: Vec<(Move, Move)> = s.trim().split("\n").map(|line| {
        let actions: Vec<Move> = line.split(' ').map(|action| {
            match action {
                "A" => return Move::Rock,
                "X" => return Move::Rock,
                "B" => return Move::Paper,
                "Y" => return Move::Paper,
                "C" => return Move::Scissors,
                "Z" => return Move::Scissors,
                _ => panic!("Can't parse the extracted file")
            }
        }).collect::<Vec<Move>>();
        let actions: (Move, Move) = (actions[0].clone(), actions[1].clone());

        actions
    }).collect();
    
    Ok(lines)
}

pub fn solve() -> (i32, i32) {
    let filepath = "inputs/day2.txt";
    let data = match extract(filepath) {
        Err(err) => panic!("We can't read from file, {}", err),
        Ok(data) => data
    };


    let mut score = 0;
    for action in &data {
        match action.1 {
            Move::Rock => {
                score += 1;
                match action.0 {
                    Move::Rock => score += 3,
                    Move::Paper => {},
                    Move::Scissors => score += 6
                }
            },
            Move::Paper => {
                score += 2;
                match action.0 {
                    Move::Rock => score += 6,
                    Move::Paper => score += 3,
                    Move::Scissors => {}
                }
            },
            Move::Scissors => {
                score += 3;
                match action.0 {
                    Move::Rock => {}
                    Move::Paper => score += 6,
                    Move::Scissors => score += 3
                }
            }
        }
    }

    let mut score2 = 0;
    for action in &data {
        match action.1 {
            // Need to lose
            Move::Rock => {
                match action.0 {
                    Move::Rock => score2 += 3,
                    Move::Paper => score2 += 1,
                    Move::Scissors => score2 += 2
                }
            },
            // Need to draw
            Move::Paper => {
                score2 += 3;
                match action.0 {
                    Move::Rock => score2 += 1,
                    Move::Paper => score2 += 2,
                    Move::Scissors => score2 += 3
                }
            },
            // Need to win
            Move::Scissors => {
                score2 += 6;
                match action.0 {
                    Move::Rock => score2 += 2,
                    Move::Paper => score2 += 3,
                    Move::Scissors => score2 += 1
                }
            }
        }
    }
    (score, score2)
}