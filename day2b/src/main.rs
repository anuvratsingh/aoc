use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

fn main() -> Result<()> {
    let file = File::open(Path::new("input.txt"))?;
    let reader = BufReader::new(file);

    let mut horizontal: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;

    // Can be done with regex but I don't know
    for line in reader.lines() {
        let newline = line?.parse::<String>().unwrap();
        let array: Vec<&str> = newline.split(" ").collect();
        let value = array[1].parse::<usize>().unwrap();

        match array[0] {
            "forward" => {
                if aim != 0 {
                    depth = depth + (value * aim);
                    horizontal = horizontal + value;
                } else {
                    horizontal = horizontal + value
                }
            }
            "up" => aim = aim - value,
            "down" => aim = aim + value,
            _ => panic!("Value not supported"),
        };
    }
    // println!("h:{} d:{}, aim:{}", horizontal, depth, aim);
    println!("Result: {}", horizontal * depth);

    Ok(())
}
