use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, self};
use std::path::Path;

// Can panic (parse)
fn extract<P: AsRef<Path>>(path: P) -> Result<Vec<char>, io::Error> {
    let mut input = File::open(path)?;

    let mut s = String::new();
    input.read_to_string(&mut s)?;

    Ok(s.chars().collect())
}

fn has_duplicate(list: Vec<char>) -> bool {
    let mut uniq = HashSet::new();
    
    for c in list {
        if uniq.contains(&c) {
            return true
        }
        uniq.insert(c);
    }
    false
}

pub fn solve() -> (i32, i32) {
    let filepath = "inputs/day6.txt";
    let data = extract(filepath)
        .expect("We can't read from file"); 

    (0, 0)
}