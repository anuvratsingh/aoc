use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn main() -> Result<()> {
    // let file = File::open(Path::new("test.txt"))?; 
    let file = File::open(Path::new("input.txt"))?;
    let reader = BufReader::new(file);

    // let each_input_len = 5; // for test, can't find a way to create an array of zeros with len equal to input
    let each_input_len = 12; // length of each input
    let mut num_of_input = 0;
    let mut counting_zero: Vec<usize> = vec![0; each_input_len];

    let mut gamma_rate_binary = String::new();
    let mut epsilon_rate_binary = String::new();

    for line in reader.lines() {
        let new_line = line?.parse::<String>().unwrap();

        for i in 0..new_line.len() {
            let ones_zeros_char = new_line.as_bytes()[i] as char;
            // println!("{}", ones_zeros_char as usize);
            if ones_zeros_char == 48 as char {
                // 48 is zero
                counting_zero[i] = counting_zero[i] + 1;
            }
        }
        num_of_input = num_of_input + 1;
        // println!("{:?}", new_line);
        // println!("first: {}", new_line.as_bytes()[0] as char);
    }

    for i in 0..counting_zero.len() {
        if counting_zero[i] > num_of_input / 2 {
            gamma_rate_binary.push(48 as char);
            epsilon_rate_binary.push(49 as char);
        } else {
            gamma_rate_binary.push(49 as char);
            epsilon_rate_binary.push(48 as char);
        }
    }
    // println!("result: {:?}, 0 as char:{};", counting_zero, 48 as char);

    let gamma_rate_decimal = usize::from_str_radix(gamma_rate_binary.as_str(), 2).unwrap();
    let epsilon_rate_decimal = usize::from_str_radix(epsilon_rate_binary.as_str(), 2).unwrap();
    println!("Result: {}", gamma_rate_decimal * &epsilon_rate_decimal);
    Ok(())
}
