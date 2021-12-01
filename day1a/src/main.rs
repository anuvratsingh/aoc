use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
fn main() -> io::Result<()> {
    // let file = fs::read_to_string(Path::new("input.txt"));

    // Opening file and adding buffer
    let file = File::open(Path::new("input.txt"))?;
    let reader = BufReader::new(file);

    // Setting up variables
    let mut num: usize = 0;
    let mut result: usize = 0;

    // loop
    for line in reader.lines() {
        let newline = line?.parse::<usize>().unwrap();
        if num == 0 {
            num = newline
        }
        if newline > num {
            result = result + 1;
        }
        num = newline
    }

    println!("{}", result);

    // println!("{}", file);

    Ok(())
}
