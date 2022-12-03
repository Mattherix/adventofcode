use std::char;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, self};
use std::path::Path;

#[derive(Debug, Clone)]
struct Rucksack {
    first_compartment: Vec<char>,
    second_compartment: Vec<char>
}
impl Rucksack {
    fn new(contents: &str) -> Self {
        let items: Vec<char> = contents.chars().collect();
        Rucksack {
            first_compartment: items[0..(items.len() / 2)].to_vec(),
            second_compartment: items[(items.len() / 2)..items.len()].to_vec()
        }
    }
    
    fn find_duplicate_item(&self) -> Option<char> {
        for first in &self.first_compartment {
            for second in &self.second_compartment {
                if *first == *second {
                    return Some(*first);
                }
            }
        }

        None
    }
}

fn create_dictionary() -> HashMap<char, u8> {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut dictionary = HashMap::new();
    for letter in alphabet.chars().enumerate() {
        dictionary.insert(letter.1, (letter.0 + 1) as u8);
    }
    dictionary
}

fn extract<P: AsRef<Path>>(path: P) -> Result<Vec<Rucksack>, io::Error> {
    let mut input = File::open(path)?;

    let mut s = String::new();
    input.read_to_string(&mut s)?;

    let lines: Vec<Rucksack> = s.trim().lines().map(|line| {
        Rucksack::new(line)
    }).collect();
    
    Ok(lines)
}

pub fn solve() -> i32 {
    let filepath = "inputs/day1.txt";
    let data = extract("inputs/day3.txt")
        .expect("We can't read the file");
    
    let dictonary = create_dictionary();

    let mut priorities = 0;
    for ruckstack in data {
        let duplicate = ruckstack.find_duplicate_item().unwrap();
        priorities += dictonary[&duplicate] as i32;
    }

    priorities
}