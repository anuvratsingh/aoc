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

        // println!("Input: {:?}", input_clone[i][0]);
        // mark positions
        for j in &input_clone[i][0] {
            let mut char_array: Vec<char> = j.chars().collect();
            char_array.sort();

            if char_array.len() == 2 && temp_positiions[2].0 == 'x' && temp_positiions[5].0 == 'x' {
                temp_positiions[2].0 = char_array[0];
                temp_positiions[5].0 = char_array[1];
            }
            if char_array.len() == 3 && temp_positiions[0].0 == 'x' {
                for z in &char_array {
                    if temp_positiions[2].0 != *z && temp_positiions[5].0 != *z {
                        temp_positiions[0].0 = *z;
                    }
                }
            }
            if char_array.len() == 4 && temp_positiions[1].0 == 'x' && temp_positiions[3].0 == 'x' {
                for z in &char_array {
                    if temp_positiions[2].0 != *z && temp_positiions[5].0 != *z {
                        if temp_positiions[1].0 == 'x' {
                            temp_positiions[1].0 = *z;
                        } else if temp_positiions[3].0 == 'x' {
                            temp_positiions[3].0 = *z;
                        }
                    }
                }
            }
            if char_array.len() == 7 && temp_positiions[4].0 == 'x' && temp_positiions[6].0 == 'x' {
                for z in &char_array {
                    if temp_positiions[0].0 != *z
                        && temp_positiions[1].0 != *z
                        && temp_positiions[2].0 != *z
                        && temp_positiions[3].0 != *z
                        && temp_positiions[5].0 != *z
                    {
                        if temp_positiions[4].0 == 'x' {
                            temp_positiions[4].0 = *z;
                        } else if temp_positiions[6].0 == 'x' {
                            temp_positiions[6].0 = *z;
                        }
                    }
                }
            }
            // println!("J :{}", j);
        }
        // find num
        // println!("Temp post : {:?}", temp_positiions);
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
                        // println!("Four digit len is 4 :{:?}", four_digit);
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
