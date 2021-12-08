use std::{fs::read_to_string, path::Path};

fn main() {
    // let file = read_to_string(Path::new("test.txt")).unwrap();
    let file = read_to_string(Path::new("input.txt")).unwrap();

    let parsed_input = parse_input(&mut file.lines());

    let uniques = count_uniques(&parsed_input);

    println!("Uniques: {}", uniques);
    // println!("Parsed Input: {:?}", parsed_input);
}

fn count_uniques(input: &Vec<Vec<Vec<String>>>) -> usize {
    let mut output = 0;

    for i in 0..input.len() {
        for j in &input[i][1] {
            if j.len() == 2 || j.len() == 4 || j.len() == 3 || j.len() == 7 {
                output = output + 1;
            }
        }
    }

    output
}
fn parse_input(input: &mut std::str::Lines<'_>) -> Vec<Vec<Vec<String>>> {
    let mut output: Vec<Vec<Vec<String>>> = Vec::new();
    loop {
        match input.next() {
            Some(x) => {
                let line: Vec<Vec<String>> = x
                    .trim()
                    .split(" | ")
                    .map(|x| {
                        x.parse::<String>()
                            .unwrap()
                            .trim()
                            .split(" ")
                            .map(|y| y.parse().unwrap())
                            .collect()
                    })
                    .collect();
                output.push(line);
            }
            None => break,
        }
    }

    output
}
