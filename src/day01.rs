use std::fs::File;
use std::vec::Vec;
use std::io::{self, prelude::*, BufReader};

// in part 1 we just had to find the first and last digits
// if only one digit, the it is both first and last
pub fn part1() -> io::Result<u32> {
    let path = "src/inputs/input01.txt";
    
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    
    let mut sum: u32 = 0;
    for line in lines.iter() {
        sum += find_calibration(line);
    }

    Ok(sum)
}

// in part 2, digits spelled out were also valid
// if there was overlap, like 'eighthree' or 'sevenine',
// it should parse to 83 and 79, respectively
pub fn part2() -> io::Result<u32> {
    let path = "src/inputs/input01.txt";

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut replaced_lines: Vec<String> = Vec::new(); 

    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    
    // replace a spelled digit with the first and last
    // letters to account for overlap
    let replace = vec![
        ("one", "o1e"),
        ("two", "t2w"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9n"),
        ("zero", "z0o")
    ];
    
    // iterate over the replace vec 
    //to replace everything into digits
    for line in lines.iter_mut() {
        for (from, to) in replace.iter() {
            *line = line.replace(from, to);
        }
        replaced_lines.push(line.clone());
    }
    
    let mut sum = 0;
    for line in replaced_lines.iter() {
        sum += find_calibration(line); 
    }

    Ok(sum)

}

fn find_calibration(line: &str) -> u32 {
    let mut first;// = String::new();
    let mut last;// = String::new();

    // first
    first = "0".to_string();
    for c in line.chars() {
        if c.is_digit(10) {
            first = c.to_string();
            break;
        }
    }

    // last
    last = "0".to_string();
    for c in line.chars().rev() {
        if c.is_digit(10) {
            last = c.to_string();
            break;
        }
    }
    
    (10*(first.parse::<u32>().unwrap()) 
    + last.parse::<u32>().unwrap() ) as u32
}