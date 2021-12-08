use std::{fs::read_to_string, path::Path};

fn main() {
    // let file = read_to_string(Path::new("test.txt")).unwrap();
    // let file = read_to_string(Path::new("input.txt")).unwrap();
    let file = read_to_string(Path::new("single_test.txt")).unwrap();

    let parsed_input = parse_input(&mut file.lines());

    // let uniques = count_uniques(&parsed_input);
    let all = count_all(&parsed_input);

    println!("All :{:?}", all);
    // println!("Uniques: {}", uniques);
    // println!("Parsed Input: {:?}", parsed_input);
}
fn count_all(input: &Vec<Vec<Vec<String>>>) -> Vec<Vec<usize>> {
    let mut output: Vec<Vec<usize>> = Vec::new();
    let mut input_clone = input.clone();

    let mut temp_positiions: Vec<(char, usize)> = vec![
        ('x', 0),
        ('x', 1),
        ('x', 2),
        ('x', 3),
        ('x', 4),
        ('x', 5),
        ('x', 6),
    ];
    for i in 0..input_clone.len() {
        input_clone[i][0].sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

        println!("Input: {:?}", input_clone[i][0]);
        // mark positions
        for j in &input_clone[i][0] {
            if j.len() == 2 {
                let chars_two: Vec<char> = j.chars().collect();
                if temp_positiions[2].0 == 'x' && temp_positiions[5].0 == 'x' {
                    temp_positiions[2].0 = chars_two[0];
                    temp_positiions[5].0 = chars_two[1];
                }
                // println!("{:?}", chars[0]);
            } else if j.len() == 3 {
                let chars_three: Vec<char> = j.chars().collect();
                let (pos_2, pos_5) = (temp_positiions[2].0, temp_positiions[5].0);

                for i in chars_three {
                    if i == pos_2 || i == pos_5 {
                        continue;
                    } else if temp_positiions[0].0 == 'x' {
                        temp_positiions[0].0 = i
                    }
                }
                // println!("Positions: {:?}", temp_positiions);
            } else if j.len() == 4 {
                let chars_four: Vec<char> = j.chars().collect();
                let (pos_2, pos_5) = (temp_positiions[2].0, temp_positiions[5].0);
                for i in chars_four {
                    if i == pos_2 || i == pos_5 {
                        continue;
                    } else if temp_positiions[1].0 == 'x' {
                        // println!("Here in 1 with char : {}", i);
                        temp_positiions[1].0 = i;
                    } else if temp_positiions[3].0 == 'x' {
                        // println!("Here in 3 with char : {}", i);
                        temp_positiions[3].0 = i;
                    }
                }
                // println!("Chars : {:?}", chars);
                // println!("Positions: {:?}", temp_positiions);
            } else if j.len() == 5 {
                let chars_five: Vec<char> = j.chars().collect();
                let (pos_0, pos_2, pos_3, pos_5, pos_6) = (
                    temp_positiions[0].0,
                    temp_positiions[2].0,
                    temp_positiions[3].0,
                    temp_positiions[5].0,
                    temp_positiions[6].0,
                );

                for i in chars_five {
                    // changing from 5 to 3
                    if i == pos_0 || i == pos_2 || i == pos_3 || i == pos_5 || i == pos_6 {
                        continue;
                    } else if temp_positiions[0].0 == 'x' {
                        temp_positiions[0].0 = i;
                    } else if temp_positiions[2].0 == 'x' {
                        temp_positiions[1].0 = i;
                    } else if temp_positiions[3].0 == 'x' {
                        temp_positiions[3].0 = i;
                    } else if temp_positiions[5].0 == 'x' {
                        temp_positiions[5].0 = i;
                    } else if temp_positiions[6].0 == 'x' {
                        temp_positiions[6].0 = i;
                    }
                }
            } else if j.len() == 7 {
                let chars_seven: Vec<char> = j.chars().collect();
                let (pos_0, pos_1, pos_2, pos_3, pos_5, pos_6) = (
                    temp_positiions[0].0,
                    temp_positiions[1].0,
                    temp_positiions[2].0,
                    temp_positiions[3].0,
                    temp_positiions[5].0,
                    temp_positiions[6].0,
                );
                for i in chars_seven {
                    if i == pos_0
                        || i == pos_1
                        || i == pos_2
                        || i == pos_3
                        || i == pos_5
                        || i == pos_6
                    {
                        continue;
                    } else if temp_positiions[0].0 == 'x' {
                        temp_positiions[0].0 = i;
                    } else if temp_positiions[1].0 == 'x' {
                        temp_positiions[1].0 = i;
                    } else if temp_positiions[2].0 == 'x' {
                        temp_positiions[2].0 = i;
                    } else if temp_positiions[3].0 == 'x' {
                        temp_positiions[3].0 = i;
                    } else if temp_positiions[4].0 == 'x' {
                        temp_positiions[4].0 = i;
                    } else if temp_positiions[5].0 == 'x' {
                        temp_positiions[5].0 = i;
                    } else if temp_positiions[6].0 == 'x' {
                        temp_positiions[6].0 = i;
                    }
                }
                println!("Positions: {:?}", temp_positiions);
            }
        }

        // find num
        println!("Temp post : {:?}", temp_positiions);
        let mut four_digit: Vec<usize> = Vec::new();
        for j in &input_clone[i][1] {
            let array_char: Vec<char> = j.chars().collect();

            // println!("Array Char: {:?}", array_char);

            for ij in 0..temp_positiions.len() {
                if temp_positiions[ij].0 != 'x' && ij == temp_positiions.len() - 1 {
                    // println!("Here");
                    if array_char.len() == 2 {
                        four_digit.push(1);
                    }
                    if array_char.len() == 3 {
                        four_digit.push(7)
                    }
                    if array_char.len() == 4 {
                        four_digit.push(4);
                    }
                    if array_char.len() == 5 {
                        if array_char.contains(&temp_positiions[1].0) {
                            four_digit.push(5);
                        } else if array_char.contains(&temp_positiions[4].0) {
                            four_digit.push(2)
                        } else if array_char.contains(&temp_positiions[5].0) {
                            four_digit.push(3)
                        }
                    }
                    if array_char.len() == 6 {
                        if !array_char.contains(&temp_positiions[3].0) {
                            four_digit.push(0)
                        }
                        if !array_char.contains(&temp_positiions[2].0) {
                            four_digit.push(6)
                        }
                        if !array_char.contains(&temp_positiions[4].0) {
                            four_digit.push(9)
                        }
                    }
                    if array_char.len() == 7 {
                        four_digit.push(8);
                    }

                    // println!("four digit: {:?}", four_digit);

                    if four_digit.len() == 4 {
                        // pushing four digit
                        println!("Four digit len is 4 :{:?}", four_digit);
                        output.push(four_digit);
                        four_digit = Vec::new();
                    }
                }
            }
        }
    }
    temp_positiions = vec![
        ('x', 0),
        ('x', 1),
        ('x', 2),
        ('x', 3),
        ('x', 4),
        ('x', 5),
        ('x', 6),
    ];
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
