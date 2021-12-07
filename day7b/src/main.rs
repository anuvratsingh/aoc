use itertools::{self, Itertools};
use std::{fs::read_to_string, path::Path};

fn main() {
    // let file = read_to_string(Path::new("test.txt")).unwrap();
    let file = read_to_string(Path::new("input.txt")).unwrap();

    let parsed_input = parse_input(&mut file.lines());
    // let array_with_count = array_with_count(parsed_input);
    // let median = median(&parsed_input);
    let mean = mean(&parsed_input);


    // let result = median_diff(parsed_input, median);
    let result = mean_diff(parsed_input, mean);

    // println!("Mean: {}", mean);
    println!("Result: {}", result);
    // println!("Parsed Input: {:?}", parsed_input);
    // println!("Median: {}", median)
    // println!("Input with count: {:?}", array_with_count);
}
fn mean_diff(input: Vec<usize>, mean: usize) -> usize {
    let mut subtracted_array: Vec<usize> = Vec::new();
    let mut output = 0;
    for i in input {
        let num_to_push = (mean as isize - i as isize).abs() as usize;
        subtracted_array.push(num_to_push);
    }

    for l in subtracted_array {
        let num_to_add = ((l*(l+1)) as f32 / 2 as f32) as usize; 
        output = output + num_to_add;
    }

    output
}

fn mean(input: &Vec<usize>) -> usize {
    let mut output: usize = 0;
    for i in input {
        output = output + i;
    }
    return (output as f32 / input.len() as f32).floor() as usize; // too high so floor
}
fn _median_diff(input: Vec<usize>, median: usize) -> usize {
    let mut subtracted_array: Vec<usize> = Vec::new();
    let mut output = 0;
    for i in input {
        let num_to_push = (median as isize - i as isize).abs() as usize;
        subtracted_array.push(num_to_push);
    }

    for j in subtracted_array {
        output = output + j;
    }
    output
}
fn _median(input: &Vec<usize>) -> usize {
    if input.len() % 2 == 0 {
        input[input.len() / 2]
    } else {
        let temp: f32 = input.len() as f32 / 2 as f32;
        (input[temp.floor() as usize] + input[temp.ceil() as usize]) / 2
    }
}
fn _array_with_count(input: Vec<usize>) -> Vec<Vec<usize>> {
    let input_to_iter = input.clone();
    let temp_input: Vec<&usize> = input.iter().unique().collect_vec();
    let mut output: Vec<Vec<usize>> = Vec::new();

    for i in temp_input {
        output.push(vec![*i, 0])
    }

    for l in 0..input_to_iter.len() {
        for j in 0..output.len() {
            if output[j][0] == input_to_iter[l] {
                output[j][1] = output[j][1] + 1;
            }
        }
    }
    output
}

fn parse_input(input: &mut std::str::Lines<'_>) -> Vec<usize> {
    let mut output: Vec<usize> = input
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|f| f.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    output.sort();
    output
}
