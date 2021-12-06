use std::{fs::read_to_string, path::Path};

fn main() {
    let file = read_to_string(Path::new("test.txt")).unwrap();
    // let file = read_to_string(Path::new("input.txt")).unwrap();

    let duration = 80;

    let mut parsed_input = parse_input(&mut file.lines());

    reproduce(&mut parsed_input, duration);

    println!("Result: {}", parsed_input.len());
    // println!("Parse Input: {:?}", parsed_input);
}

fn reproduce(input: &mut Vec<usize>, duration: usize) -> &mut Vec<usize> {
    for _ in 0..duration {
        for i in 0..input.len() {
            if input[i] != 0 {
                input[i] = input[i] - 1;
            } else if input[i] == 0 {
                input.push(8);
                input[i] = 6;
            }
        }
    }
    input
}
fn parse_input(input: &mut std::str::Lines<'_>) -> Vec<usize> {
    let output = input
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    // loop {
    //     let next_line = input.next();

    //     match next_line {
    //         Some(line) => {
    //             output = line
    //                 .trim()
    //                 .split(",")
    //                 .map(|x| x.parse::<usize>().unwrap())
    //                 .collect();
    //         }
    //         None => break,
    //     }
    // }

    output
}
