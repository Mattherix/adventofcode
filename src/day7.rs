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

    (0, 0)
}

#[test]
fn feature() {
    assert_eq!(solve("inputs/day7_test.txt"), (95437, 0));
}