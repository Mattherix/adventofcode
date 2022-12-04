use std::fs::File;
use std::io::{Read, self};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Range {
    lower_bound: i32,
    higher_bound: i32
}

impl Range {
    fn new(input: &str) -> Self {
        let bounds: Vec<i32> = input
            .split("-")
            .map(|bound| bound.parse::<i32>().unwrap())
            .collect();
        
        Range {
            lower_bound: bounds[0],
            higher_bound: bounds[1]
        }
    }
}

// Can panic (parse)
fn extract<P: AsRef<Path>>(path: P) -> Result<Vec<(Range, Range)>, io::Error> {
    let mut input = File::open(path)?;

    let mut s = String::new();
    input.read_to_string(&mut s)?;

    let ranges_pairs: Vec<(Range, Range)> = s
        .trim()
        .split("\n")
        .map(|line| {
            let pairs: Vec<Range> = line
                .split(",")
                .map(|range| Range::new(range))
                .collect();
            
            (pairs[0], pairs[1])
        }).collect();
    
    Ok(ranges_pairs)
}

pub fn solve() -> i32 {
    let filepath = "inputs/day4.txt";
    let data = extract(filepath)
        .expect("We can't read from file"); 

    dbg!(data);

    0
}