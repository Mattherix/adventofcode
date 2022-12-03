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

    fn get_all_items(&self) -> Vec<char> {
        [self.first_compartment.clone(), self.second_compartment.clone()].concat()
    }
}

#[derive(Debug, Clone)]
struct Group {
    first_rucksack: Rucksack,
    second_rucksack: Rucksack,
    third_rucksack: Rucksack
}

impl Group {
    fn new(first: Rucksack, second: Rucksack, third: Rucksack) -> Self {
        Group {
            first_rucksack: first,
            second_rucksack: second,
            third_rucksack: third
        }
    }
    
    fn find_badge(&self) -> Option<char> {
        for first in self.first_rucksack.get_all_items() {
            for second in self.second_rucksack.get_all_items() {
                for third in self.third_rucksack.get_all_items() {
                    if first == second && second == third {
                        return Some(first);
                    }
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

pub fn solve() -> (i32, i32) {
    let data = extract("inputs/day3.txt")
        .expect("We can't read the file");
    
    let dictonary = create_dictionary();

    let mut priorities = 0;
    for ruckstack in &data {
        let duplicate = ruckstack.find_duplicate_item().unwrap();
        priorities += dictonary[&duplicate] as i32;
    }

    let slices: Vec<&[Rucksack]> = data.chunks(3).collect();
    let mut badges = 0;
    for chunks in slices {
        let group = Group::new(
            chunks[0].clone(),
            chunks[1].clone(),
            chunks[2].clone()
        );
        let badge = group.find_badge().unwrap();
        badges += dictonary[&badge] as i32;
    }

    (priorities, badges)
}