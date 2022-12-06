use std::fs::File;
use std::io::{Read, self};
use std::path::Path;

// Can panic (parse)
fn extract<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<i32>>, io::Error> {
    let mut input = File::open(path)?;

    let mut s = String::new();
    input.read_to_string(&mut s)?;

    let lines  = s.trim().split("\n\n").map(|line| {
        line.split('\n').map(|number| {
            number.parse::<i32>().unwrap()
        }).collect::<Vec<i32>>()
    }).collect();
    
    Ok(lines)
}

pub fn solve() -> (i32, i32, i32) {
    let filepath = "inputs/day1.txt";
    let data = match extract(filepath) {
        Err(err) => panic!("We can't read from file, {}", err),
        Ok(data) => data
    };
    
    let elves: Vec<i32> = data.iter().map(|elf| elf.iter().sum()).collect();

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;  
    for calories in elves {
        if calories > first {
            third = second;
            second = first;
            first = calories;
            continue;
        }
        if calories > second {
            third = second;
            second = calories;
            continue;
        }
        if calories > third {
            third = calories
        }
    }

    (first, second, third)
}