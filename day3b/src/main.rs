use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn main() -> Result<()> {
    // Reading file again
    let file = File::open(Path::new("input.txt"))?;
    let reader = BufReader::new(file);
    let mut input_array: Vec<String> = Vec::new();
    for line in reader.lines() {
        input_array.push(line?.parse::<String>().unwrap());
    }

    let (oxygen_generator_rating, _) = find_max_min(input_array.clone(), MaxMin::Max, None);
    let (co2_scrubber_rating, _) = find_max_min(input_array.clone(), MaxMin::Min, None);

    let oxygen_generator_rating_decimal =
        usize::from_str_radix(oxygen_generator_rating[0].as_str(), 2).unwrap();
    let co2_scrubber_rating_decimal =
        usize::from_str_radix(co2_scrubber_rating[0].as_str(), 2).unwrap();

    println!(
        "Result: {}",
        oxygen_generator_rating_decimal * co2_scrubber_rating_decimal
    );

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

fn find_max_min(
    input: Vec<String>,
    interest: MaxMin,
    iter_input: Option<usize>,
) -> (Vec<String>, Option<usize>) {
    let iter: usize;
    match iter_input {
        Some(_) => {
            iter = iter_input.unwrap();
        }
        None => {
            iter = 0;
        }
    }

    if input.len() > 1 {
        match interest {
            MaxMin::Max => {
                let mut max_output: Vec<String> = Vec::new();
                let mut zero = 0;
                let mut one = 0;

                for element_one in &input {
                    if element_one.as_bytes()[iter] as char == 48 as char {
                        zero = zero + 1;
                    } else {
                        one = one + 1;
                    }
                }
                if zero > one {
                    for element_two in input {
                        if element_two.as_bytes()[iter] as char == 48 as char {
                            max_output.push(element_two);
                        }
                    }
                } else if zero == one {
                    for element_two in input {
                        if element_two.as_bytes()[iter] as char == 49 as char {
                            max_output.push(element_two)
                        }
                    }
                } else {
                    for element_two in input {
                        if element_two.as_bytes()[iter] as char == 49 as char {
                            max_output.push(element_two);
                        }
                    }
                }

                find_max_min(max_output, MaxMin::Max, Some(iter + 1))
            }
            MaxMin::Min => {
                let mut min_output: Vec<String> = Vec::new();
                let mut zero = 0;
                let mut one = 0;

                for element_one in &input {
                    if element_one.as_bytes()[iter] as char == 48 as char {
                        zero = zero + 1;
                    } else {
                        one = one + 1;
                    }
                }
                if zero < one {
                    for element_two in input {
                        if element_two.as_bytes()[iter] as char == 48 as char {
                            min_output.push(element_two);
                        }
                    }
                } else if zero == one {
                    for element_two in input {
                        if element_two.as_bytes()[iter] as char == 48 as char {
                            min_output.push(element_two);
                        }
                    }
                } else {
                    for element_two in input {
                        if element_two.as_bytes()[iter] as char == 49 as char {
                            min_output.push(element_two);
                        }
                    }
                }

                find_max_min(min_output, MaxMin::Min, Some(iter + 1))
            }
        }
    } else {
        (input, None)
    }
}
