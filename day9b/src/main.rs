use std::{fs::read_to_string, path::Path};

fn main() {
    // let file = read_to_string(Path::new("test.txt")).unwrap();
    let file = read_to_string(Path::new("input.txt")).unwrap();
    let parsed_input = parser(&mut file.lines());
    let array_of_minimas = all_local_minimas(&parsed_input);
    let result = find_sum(&parsed_input, array_of_minimas);

    println!("Result: {}", result);
    // println!("All minimas: {:?}", array_of_minimas);
    // println!("Parsed Input: {:?}", parsed_input);
}
fn find_sum(input_n2: &Vec<Vec<u8>>, minimas: Vec<(usize, usize)>) -> usize {
    let mut output: usize = 0;
    for i in 0..minimas.len() {
        output = output + input_n2[minimas[i].0][minimas[i].1] as usize + 1;
    }

    output
}
fn all_local_minimas(input: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut local_minimas: Vec<(usize, usize)> = Vec::new();

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if i > 0 && j > 0 && i < input.len() - 1 && j < input[i].len() - 1 {
                if input[i][j] < input[i - 1][j]
                    && input[i][j] < input[i + 1][j]
                    && input[i][j] < input[i][j - 1]
                    && input[i][j] < input[i][j + 1]
                {
                    local_minimas.push((i, j));
                }
            }
            if i > 0 && i < input.len() - 1 {
                if j == 0 {
                    if input[i][j] < input[i - 1][j]
                        && input[i][j] < input[i + 1][j]
                        && input[i][j] < input[i][j + 1]
                    {
                        local_minimas.push((i, j))
                    };
                }
                if j == input[i].len() - 1 {
                    if input[i][j] < input[i - 1][j]
                        && input[i][j] < input[i + 1][j]
                        && input[i][j] < input[i][j - 1]
                    {
                        local_minimas.push((i, j))
                    };
                }
            }
            if j > 0 && j < input[i].len() - 1 {
                if i == 0 {
                    if input[i][j] < input[i][j - 1]
                        && input[i][j] < input[i][j + 1]
                        && input[i][j] < input[i + 1][j]
                    {
                        local_minimas.push((i, j))
                    };
                }
                if i == input.len() - 1 {
                    if input[i][j] < input[i][j - 1]
                        && input[i][j] < input[i][j + 1]
                        && input[i][j] < input[i - 1][j]
                    {
                        local_minimas.push((i, j))
                    };
                }
            }
            if i == 0 && j == 0 {
                if input[i][j] < input[i + 1][j] && input[i][j] < input[i][j + 1] {
                    local_minimas.push((i, j));
                }
            }
            if i == input.len() - 1 && j == input[i].len() - 1 {
                if input[i][j] < input[i - 1][j] && input[i][j] < input[i][j - 1] {
                    local_minimas.push((i, j));
                }
            }
            if i == 0 && j == input[i].len() - 1 {
                if input[i][j] < input[i][j - 1] && input[i][j] < input[i + 1][j] {
                    local_minimas.push((i, j));
                }
            }
            if j == 0 && i == input.len() - 1 {
                if input[i][j] < input[i - 1][j] && input[i][j] < input[i][j + 1] {
                    local_minimas.push((i, j));
                }
            }
        }
    }

    local_minimas
}
fn parser(input: &mut std::str::Lines) -> Vec<Vec<u8>> {
    let mut output: Vec<Vec<u8>> = Vec::new();

    loop {
        match input.next() {
            Some(x) => {
                let one_d: Vec<char> = x.trim().chars().collect();
                // .split("")
                // .map(|f| f.parse::<usize>().unwrap())
                // .collect();
                let char_to_u8: Vec<u8> = one_d
                    .iter()
                    .map(|f| f.to_digit(10).unwrap() as u8)
                    .collect();
                // println!("ond_d: {:?}", one_d);
                output.push(char_to_u8);
            }
            None => break,
        }
    }
    output
}
