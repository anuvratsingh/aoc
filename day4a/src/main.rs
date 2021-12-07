#[allow(dead_code, unused_assignments, unused_mut)]
use std::{fs::read_to_string, path::Path};

fn main() {
    // let file = read_to_string(Path::new("test.txt")).unwrap();
    let file = read_to_string(Path::new("input.txt")).unwrap();

    let (bingo_input, mut bingo_board_s) = parse_input(&mut file.lines());

    let mut result = 0;
    for i in bingo_input {
        let mut condition = false;
        for j in 0..bingo_board_s.len() {
            mark_board(i, &mut bingo_board_s[j]);
            if check_bingo(&bingo_board_s[j]) {
                println!("Bingo: {:?}", bingo_board_s[j]);
                let temp = add_umarked(&bingo_board_s[j]);
                println!("I: {}, temp: {}", i, temp);
                result = i * temp;
                condition = true;
                break;
            }
        }
        if condition {
            break;
        }
    }
    println!("Result: {}", result);
    // println!("Bingo Input: {:?}", bingo_input);
    // println!("Bingo Boards: {:?}", bingo_board_s);
}
fn add_umarked(input: &Vec<Vec<(usize, bool)>>) -> usize {
    let mut output = 0;
    for i in 0..5 {
        for j in 0..5 {
            if input[i][j].1 == false {
                output = output + input[i][j].0;
            }
        }
    }
    output
}
fn check_bingo(bingo_board: &Vec<Vec<(usize, bool)>>) -> bool {
    // row
    // println!("In check_bingo");
    for i in 0..5 {
        for j in 0..5 {
            if bingo_board[i][j].1 == false {
                break;
            } else if bingo_board[i][j].1 == true && j == 4 {
                return true;
            }
        }
    }
    // column
    for i in 0..5 {
        for j in 0..5 {
            if bingo_board[j][i].1 == false {
                break;
            }  else if bingo_board[j][i].1 == true && j == 4 {
                return true;
            }
        }
    }
    return false;
}
fn mark_board(bingo_input: usize, bingo_board: &mut Vec<Vec<(usize, bool)>>) {
    for l in 0..5 {
        for m in 0..5 {
            if bingo_board[l][m].0 == bingo_input {
                bingo_board[l][m].1 = true
            }
        }
    }
}

fn parse_input(input: &mut std::str::Lines<'_>) -> (Vec<usize>, Vec<Vec<Vec<(usize, bool)>>>) {
    let output_one: Vec<usize> = input
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut output_two: Vec<Vec<Vec<(usize, bool)>>> = Vec::new();
    let mut temp_as_arrays: Vec<Vec<(usize, bool)>> = Vec::new();
    let mut iter = 0;
    loop {
        match input.next() {
            Some(x) => {
                if x != "" {
                    let temp_array = x
                        .trim()
                        .split_ascii_whitespace()
                        .map(|f| (f.parse::<usize>().unwrap(), false))
                        .collect();
                    // println!("Temp Array: {:?}", temp_array);
                    temp_as_arrays.push(temp_array);
                    iter = iter + 1;
                    // println!("Iter: {}", iter);
                    // println!("Temp board: {:?}", temp_board);
                }
            }
            None => break,
        }
    }
    let mut temp: Vec<Vec<(usize, bool)>> = Vec::new();
    let mut i = 0;

    loop {
        if i < temp_as_arrays.len() {
            if temp.len() < 5 {
                temp.push(temp_as_arrays[i].clone());
                i = i + 1;
                if i == temp_as_arrays.len() {
                    output_two.push(temp);
                    break;
                }
                // println!("I1: {}", i)
            } else if temp.len() == 5 {
                // println!("I2: {}", i);
                output_two.push(temp);
                temp = Vec::new();
            }
        } else {
            break;
        }
    }
    // println!("Output two: {:?}", output_two);
    (output_one, output_two)
}
