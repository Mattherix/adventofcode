use std::{path::Path, io::{self, Read}, fs::File};

fn extract<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<String>>, io::Error> {
    let mut input = File::open(path)?;

    let mut s = String::new();
    input.read_to_string(&mut s)?;

    let commands: Vec<Vec<String>> = s
        .split("$ ")
        .skip(1)
        .map(|command| {
            command
                .split_whitespace()
                .map(str::to_string)
                .collect()
        })
        .collect();
    
    Ok(commands)
}

pub fn solve<P: AsRef<Path>>(path: P) -> (u32, u32) {
    let data = extract(path)
        .expect("We can't read the file");
    
    let data:Vec<Vec<&str>> = data
        .iter()
        .map(|instruction| {
            instruction
                .iter()
                .map(|s| s.as_str())
                .collect()
        })
        .collect();

    let mut directory_size = vec![];
    let mut directory_size_stack = vec![];
    for instruction in data {
        match instruction[0] {
            "cd" => {
                match instruction[1] {
                    ".." => {
                        if let Some(size) = directory_size_stack.pop() {
                            let mut last = directory_size_stack.last_mut().unwrap();
                            *last += size;

                            directory_size.push(size);
                        }
                    },
                    name => directory_size_stack.push(0)
                }
                
            },
            "ls" => {
                let mut last = directory_size_stack.last_mut().unwrap();
                
                *last += instruction
                    .iter()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>();
            },
            _ => panic!("Can't execute instruction {:?}", instruction)
        }
    }
    
    while let Some(size) = directory_size_stack.pop() {
        directory_size.push(size);
    }
    
    let part1 = directory_size
        .iter()
        .filter(|val| **val <= 100_000)
        .sum();

    (part1, 0)
}

#[test]
fn feature() {
    assert_eq!(solve("inputs/day7_test.txt"), (95437, 0));
}