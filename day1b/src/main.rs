use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
fn main() -> io::Result<()> {
    // let file = fs::read_to_string(Path::new("input.txt"));

    // Opening file and adding buffer
    let file = File::open(Path::new("input.txt"))?;
    let reader = BufReader::new(file);

    // Setting up variables
    let mut num_array: Vec<usize> = Vec::new();
    let mut new_array: Vec<usize> = Vec::new();

    let mut result: usize = 0;

    // loop making an array from input
    for line in reader.lines() {
        num_array.push(line?.parse::<usize>().unwrap())
    }

    // Making new array with three input
    for n in 0..num_array.len() - 2 {
        new_array.push(num_array[n] + num_array[n + 1] + num_array[n + 2]);
        // println!("{}", num_array[n])
    }

    // finding the number of increments
    for n in 1..new_array.len() {
        if new_array[n] > new_array[n - 1] {
            result = result + 1;
        }
    }

    println!("{}", result);

    Ok(())
}
