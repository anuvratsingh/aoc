use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

fn main() -> Result<()> {
    let file = File::open(Path::new("input.txt"))?;
    let reader = BufReader::new(file);

    let mut horizontal : usize = 0;
    let mut depth : usize= 0;

    for line in reader.lines() {
        let newline = line?.parse::<String>().unwrap();
        let array: Vec<&str> = newline.split(" ").collect();
        let value = array[1].parse::<usize>().unwrap();

        match array[0] {
            "forward" => horizontal = horizontal + value,
            "up" => depth = depth - value,
            "down" => depth = depth + value,
            _ => panic!("Value not supported"),
        };
        // println!("h:{} d:{}", horizontal, depth);
    }
    println!("{}", horizontal * depth );

    Ok(())
}
