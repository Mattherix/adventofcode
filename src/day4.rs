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
    fn is_fully_contained(&self, possible_subset: Range) -> bool {
        if self.lower_bound >= possible_subset.lower_bound && self.higher_bound <= possible_subset.higher_bound {
            true
        } else {
            false
        }
    }
    fn is_overlapping_at_all(&self, possible_subset: Range) -> bool {
        if self.higher_bound >= possible_subset.lower_bound && self.lower_bound <= possible_subset.higher_bound {
            true
        } else {
            false
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

pub fn solve() -> (i32, i32) {
    let filepath = "inputs/day4.txt";
    let data = extract(filepath)
        .expect("We can't read from file"); 

    let mut pairs_fully_contained = 0;
    let mut pairs_overlapping_at_all = 0;
    for pair in data {
        if pair.0.is_fully_contained(pair.1) | pair.1.is_fully_contained(pair.0) {
            pairs_fully_contained += 1;
        }
        if pair.0.is_overlapping_at_all(pair.1) | pair.1.is_overlapping_at_all(pair.0) {
            pairs_overlapping_at_all += 1;
        }
    }

    (pairs_fully_contained, pairs_overlapping_at_all)
}