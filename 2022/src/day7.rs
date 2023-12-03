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
    let mut space_used: u32 = 0;
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
                
                let files_size = instruction
                    .iter()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>();

                space_used += &files_size;
                *last += files_size
            },
            _ => panic!("Can't execute instruction {:?}", instruction)
        }
    }
    
    while let Some(size) = directory_size_stack.pop() {
        let mut last = directory_size_stack.last_mut();
        if let Some(last) = last {
            *last += size;
        }

        directory_size.push(size);
    }
    
    let part1 = directory_size
        .iter()
        .filter(|val| **val <= 100_000)
        .sum();
    
    let free_space = 70_000_000 - space_used as i32;
    let space_to_free = 30_000_000 - free_space;
    
    let mut mini = std::u32::MAX;
    for directory in directory_size {
        if directory >= space_to_free.try_into().unwrap() && directory <= mini {
            mini = directory;
        }
    }

    (part1, mini)
}

#[test]
fn test_example_input() {
    assert_eq!(solve("inputs/day7_test.txt"), (95437, 24933642));
}
