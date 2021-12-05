use std::{fs::read_to_string, path::Path};

fn main() {
    let file = read_to_string(Path::new("test.txt")).unwrap();
    // let file = read_to_string(Path::new("input.txt")).unwrap();

    let mut new_board: Vec<Vec<usize>> = vec![vec![0; 10]; 10]; //10 by 10
                                                                // let mut new_board: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000]; //1000 by 1000

    // println!("{:?}", nine_by_nine);

    let parsed_input = parse_input(&mut file.lines());

    make_board(&parsed_input, &mut new_board);
    println!("Board: {:?}", new_board);

    // println!("Parsed Input: {:?}", parsed_input);
    // println!("Grid: {:?}", nine_by_nine);
}

fn make_board(input: &Vec<Vec<usize>>, board: &mut Vec<Vec<usize>>) {
    for i in 0..input.len() {
        let (x1, y1, x2, y2) = (input[i][0], input[i][1], input[i][2], input[i][3]); // find better way to destructre
        if x1 == x2 {
            if y1 > y2 {
                for l in y2..y1 + 1 {
                    // +1 to include y1
                    board[l][x1] = board[l][x1] + 1;
                    // println!("Changed board y1>y2");
                }
                // println!("y1:{} is greater than y2:{}", y1, y2)
            } else if y1 < y2 {
                for l in y1..y2 + 1 {
                    // +1 to include y2
                    board[l][x1] = board[l][x1] + 1;
                    // println!("Changed board y1<y2");
                }
                // println!("y1:{} is less than y2:{}", y1, y2)
            } else if y1 == y2 {
                board[x1][y1] = board[x1][y1] + 1;
                // println!("Changed board y1=y2");
                // println!("y1:{} is equal to y2:{}", y1, y2)
            }
            // println!("x1 is equal to x2 in iter: {}", i);
        } else if y1 == y2 {
            if x1 > x2 {
                for l in x2..x1 + 1 { // +1 to include x1
                    board[y1][l] = board[y1][l] + 1;
                    // println!("Changed board x1>x2");
                }
            } else if x1 < x2 {
                for l in x1..x2 { // +1 to include x2
                    board[y1][l] = board[y1][l] + 1;
                    // println!("Changed board x1<x2");
                }
            } else if x1 == x2 {
                board[x1][y1] = board[x1][y1] + 1;
                // println!("Changed board x1=x2");
            }
        } else {
            continue;
        }
    }
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
