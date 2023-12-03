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

fn has_duplicate(list: &[char]) -> bool {
    let mut uniq = HashSet::new();
    
    for c in list {
        if !uniq.insert(c) {
            return true
        }
    }
    false
}


pub fn solve() -> (i32, i32) {
    let filepath = "inputs/day6.txt";
    let data = extract(filepath)
        .expect("We can't read from file"); 


    let marker: usize = data
        .windows(4)
        .enumerate()
        .find_map(|(index, window)| {
            if has_duplicate(window) {
                None
            } else {
                Some(index)
            }
        }).unwrap() + 4;
    
    let start_marker: usize = data
        .windows(14)
        .enumerate()
        .find_map(|(index, window)| {
            if has_duplicate(window) {
                None
            } else {
                Some(index)
            }
        }).unwrap() + 14;
    
    (marker as i32, start_marker as i32)
}