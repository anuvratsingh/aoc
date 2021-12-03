use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn main() -> Result<()> {
    // Reading file again
    let file = File::open(Path::new("test.txt"))?;
    let reader = BufReader::new(file);
    let mut input_array: Vec<String> = Vec::new();
    for line in reader.lines() {
        input_array.push(line?.parse::<String>().unwrap());
    }

    find_max_min(input_array.clone(), MaxMin::Max);
    find_max_min(input_array.clone(), MaxMin::Min);

    Ok(())
}

// Need a recurisve function for it
// takes an array as input, number you are intrested in either Max or Min
// loop through the array and store max and min occurence of 1 and 0 for that element
// Then move max number in an array while min in another
// Keep doing it until number of element in both array is one
// Return max min

enum MaxMin {
    Max,
    Min,
}

fn find_max_min(input: Vec<String>, interest: MaxMin) -> Vec<String> {
    if input.len() > 1 {
        match interest {
            MaxMin::Max => {
                let mut max_output: Vec<String> = Vec::new();
                let mut zero = 0;
                let mut one = 0;

                while one < input.len() / 2 || zero < input.len() / 2 {
                    for i in input {
                        for j in input[i] {}
                    }
                }
                max_output
            }
            MaxMin::Min => {
                println!("Min {:?}", input);
            }
        }
    } else {
        input
    }
}
