use std::{fs::read_to_string, path::Path};

fn main() {
    // let file = read_to_string(Path::new("test.txt")).unwrap();
    let file = read_to_string(Path::new("input.txt")).unwrap();

    let mut _nine_by_nine: Vec<Vec<usize>> = vec![vec![0; 10]; 10];

    // println!("{:?}", nine_by_nine);

    let parsed_input = parse_input(&mut file.lines());

    println!("Parsed Input: {:?}", parsed_input);
    // println!("Grid: {:?}", nine_by_nine);
}

fn parse_input(input: &mut std::str::Lines<'_>) -> Vec<Vec<usize>> {
    let mut output: Vec<Vec<usize>> = Vec::new();
    loop {
        let next_line = input.next();

        match next_line {
            Some(line) => {
                let some_new_line_string = line
                    .trim()
                    .split(" -> ")
                    .map(|x| {
                        x.split(",")
                            .map(|i| i.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<Vec<usize>>>()
                    .concat();

                // for i in 0..some_new_line_string.len() {
                // println!("{:?}", some_new_line_string);
                output.push(some_new_line_string);
                // }

                // if some_new_line_string[0] == some_new_line_string[2] {
                //     for l in some_new_line_string[0]..some_new_line_string[2]  {
                //         array_to_draw_on[some_new_line_string[0]][l] =
                //             array_to_draw_on[some_new_line_string[0]][l] + 1
                //     }
                // } else if some_new_line_string[1] == some_new_line_string[3] {
                //     for l in some_new_line_string[0]..some_new_line_string[2]+ 1 {
                //         array_to_draw_on[l][some_new_line_string[1]] =
                //             array_to_draw_on[l][some_new_line_string[1]] + 1;
                //     }
                // }
            }
            None => break,
        }
    }
    output
}
