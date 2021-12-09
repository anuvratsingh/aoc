use std::{fs::read_to_string, path::Path};

fn main() {
    let file = read_to_string(Path::new("test.txt")).unwrap();
    // let file = read_to_string(Path::new("input.txt")).unwrap();
    // let file = read_to_string(Path::new("single_test.txt")).unwrap();

    let parsed_input = parse_input(&mut file.lines());

    // let uniques = count_uniques(&parsed_input);
    let all = count_all(&parsed_input);

    println!("All :{:?}", all);
    let mut result = 0;
    for mut i in all {
        let mut _temp_string = 0;
        i.reverse();
        let base: i32 = 10;
        for j in 0..i.len() {
            if j == 0 {
                _temp_string = _temp_string + i[0];
            } else {
                _temp_string = _temp_string + (i[j] * base.pow(j as u32) as usize);
            }
        }
        result = result + _temp_string;
        _temp_string = 0;
    }
    println!("Result, {}", result);
    // println!("Uniques: {}", uniques);
    // println!("Parsed Input: {:?}", parsed_input);
}
fn count_all(input: &Vec<Vec<Vec<String>>>) -> Vec<Vec<usize>> {
    let mut output: Vec<Vec<usize>> = Vec::new();
    let input_clone = input.clone();

    // finding positions
    for i in 0..input_clone.len() {
        let mut temp_four_digit: Vec<usize> = Vec::new();
        let mut temp_positiions: Vec<String> = vec![String::new(); 10];
        for j in &input_clone[i][0] {
            if j.len() == 2 {
                temp_positiions[1] = j.to_string();
            }
            if j.len() == 3 {
                temp_positiions[7] = j.to_string();
            }
            if j.len() == 4 {
                temp_positiions[4] = j.to_string();
            }
            if j.len() == 7 {
                temp_positiions[8] = j.to_string();
            }
            if j.len() == 5 && temp_positiions[1].chars().all(|f| j.contains(f)) {
                temp_positiions[3] = j.to_string();
            }
            if j.len() == 6
                && temp_positiions[4].chars().all(|f| j.contains(f))
                && temp_positiions[7].chars().all(|f| j.contains(f))
            {
                temp_positiions[9] = j.to_string();
            }
            if j.len() == 5 && !j.chars().all(|f| temp_positiions[9].contains(f)) {
                temp_positiions[2] = j.to_string();
            }
            if j.len() == 5 {
                temp_positiions[5] = j.to_string();
            }
            if j.len() == 6
                && j != &temp_positiions[9]
                && temp_positiions[5].chars().all(|f| j.contains(f))
            {
                temp_positiions[6] = j.to_string();
            }
            if j.len() == 6 {
                temp_positiions[0] = j.to_string();
            }
        }
        println!("Temp pos: {:?}", temp_positiions);
        // check output
        
        // for k in &input_clone[i][1] {
        //     println!("k: {:?}", k);
        //     if k.chars().all(|f| temp_positiions[0].contains(f)) {
        //         if temp_four_digit.len() < 4 {
        //             temp_four_digit.push(0);
        //         }
        //     }
        //     if k.chars().all(|f| temp_positiions[1].contains(f)) {
        //         if temp_four_digit.len() < 4 {
        //             temp_four_digit.push(1);
        //         }
        //     }
        //     if k.chars().all(|f| temp_positiions[2].contains(f)) {
        //         if temp_four_digit.len() < 4 {
        //             temp_four_digit.push(2);
        //         }
        //     }
        //     if k.chars().all(|f| temp_positiions[3].contains(f)) {
        //         if temp_four_digit.len() < 4 {
        //             temp_four_digit.push(3);
        //         }
        //     }
        //     if k.chars().all(|f| temp_positiions[4].contains(f)) {
        //         if temp_four_digit.len() < 4 {
        //             temp_four_digit.push(4);
        //         }
        //     }
        //     if k.chars().all(|f| temp_positiions[5].contains(f)) {
        //         if temp_four_digit.len() < 4 {
        //             temp_four_digit.push(5);
        //         }
        //     }
        //     if k.chars().all(|f| temp_positiions[6].contains(f)) {
        //         if temp_four_digit.len() < 4 {
        //             temp_four_digit.push(6);
        //         }
        //     }
        //     if k.chars().all(|f| temp_positiions[7].contains(f)) {
        //         if temp_four_digit.len() < 4 {
        //             temp_four_digit.push(7);
        //         }
        //     }
        //     if k.chars().all(|f| temp_positiions[8].contains(f)) {
        //         if temp_four_digit.len() < 4 {
        //             temp_four_digit.push(8);
        //         }
        //     }
        //     if k.chars().all(|f| temp_positiions[9].contains(f)) {
        //         if temp_four_digit.len() < 4 {
        //             temp_four_digit.push(9);
        //         }
        //     }
        // }
        
        if temp_four_digit.len() == 4 {
            output.push(temp_four_digit);
            temp_four_digit = Vec::new();
        }
    }
    // println!("output: {:?}", output);
    output
}
fn _count_uniques(input: &Vec<Vec<Vec<String>>>) -> usize {
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
