use std::fs::File;
use std::io::{Read, self};
use std::path::Path;

#[derive(Debug, Clone)]
struct Stack<T> {
    stack: Vec<T>,
    height: u32
}

#[derive(Debug, Clone, Copy)]
enum InstructionType {
    Move
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            stack: Vec::new(),
            height: 0
        }
    }

    fn push(&mut self, value: T) {
        self.stack.push(value);
        self.height += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.height -= 1;
        self.stack.pop()        
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    instruction: InstructionType,
    from: u32,
    to: u32,
    quantity: u32
}

impl Instruction {
    fn new(instruction:InstructionType , from: u32, to: u32, quantity: u32) -> Self {
        Instruction {
            instruction: instruction,
            from: from,
            to: to,
            quantity: quantity
        }
    }
}

fn extract<P: AsRef<Path>>(path: P) -> Result<(Vec<Stack<char>>, Vec<Instruction>), io::Error> {
    let mut input = File::open(path)?;

    let mut s = String::new();
    input.read_to_string(&mut s)?;

    let parts: Vec<&str> = s
        .split("\n\n")
        .collect();
    
    let (dock, instructions) = (parts[0], parts[1]);

    let lines: Vec<String> = dock
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            let s = line.to_string();
            let s = s.replace("    ", "_");
            let s = s.replace(&['[', ']', ' '][..], "");
            
            s
        })
        .collect();
    
    let mut stacks = vec![Stack::new(); lines[0].len()];
    for line in lines.iter() {
        for (index, ship_crate) in line.chars().enumerate() {
            if ship_crate != '_' {
                stacks[index].push(ship_crate);
            }
        }
    }
    
    let instructions = instructions
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line
                .split(' ')
                .collect();

            Instruction::new(
                InstructionType::Move,
                parts[3].parse::<u32>().unwrap(),
                parts[5].parse::<u32>().unwrap(),
                parts[1].parse::<u32>().unwrap()
            )
        })
        .collect();

    Ok((stacks, instructions))
}

pub fn solve() -> String {
    let filepath = "inputs/day5.txt";
    let data = extract(filepath)
        .expect("We can't read from file"); 
    
    String::from("")
}