use std::fs::File;
use std::io::Read;

fn extract(path: &str) -> Vec<Vec<i32>> {
    let mut input = match File::open(path) {
        Err(err) => panic!("couldn't open {}", err),
        Ok(file) => file
    };

    let mut s = String::new();
    match input.read_to_string(&mut s) {
        Err(err) => panic!("couldn't read {}", err),
        Ok(_) => {}
    };

    let lines: Vec<Vec<i32>> = s.trim().split("\n\n").map(|line| {
        line.split('\n').map(|number| {
            number.parse::<i32>().unwrap()
        }).collect::<Vec<i32>>()
    }).collect();
    
    lines
}

pub fn solve() -> (usize, i32) {
    let data = extract("inputs/day1.txt");
    
    let calories: Vec<i32> = data.iter().map(|elf| elf.iter().sum()).collect();

    let max = calories
        .iter()
        .enumerate()
        .map(|(index, calories)| (calories, index))
        .max()
        .unwrap();
    
    (max.1, *max.0)
}