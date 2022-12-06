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

    let mut previous = Vec::new();
    let mut marker = 0;
    
    while previous.len() < 4 || has_duplicate(previous.clone()) {
        previous.push(data[marker]);
        if previous.len() > 4 {
            previous.remove(0);
        }
        marker += 1;
    }

    let mut previous = Vec::new();
    let mut start_marker = 0;
    
    while previous.len() < 14 || has_duplicate(previous.clone()) {
        previous.push(data[start_marker]);
        if previous.len() > 14 {
            previous.remove(0);
        }
        start_marker += 1;
    }

    (marker as i32, start_marker as i32)
}