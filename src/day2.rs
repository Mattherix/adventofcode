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
    
    dbg!(&lines);

    Ok(lines)
}

pub fn solve() -> i32 {
    let filepath = "inputs/day2.txt";
    let data = match extract(filepath) {
        Err(err) => panic!("We can't read from file, {}", err),
        Ok(data) => data
    };
    0
}