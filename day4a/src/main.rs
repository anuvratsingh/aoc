use std::{fs::read_to_string, io::Result, path::Path};

fn main() -> Result<()> {
    let board_size = 5;
    let file = read_to_string(Path::new("test.txt")).unwrap(); // can't find a way to get txt as string blob from bufreader
                                                               // println!("{:?}", file.lines().next().unwrap());
                                                               // println!("{:?}", file.lines()[0]);
    let (bingo_input, bingo_boards) = make_bingo_board_and_input(&mut file.lines());

    // println!("Input: {:?}", bingo_input);

    for num_to_mark in bingo_input {
        for single_board in bingo_boards.clone() {
            for single_board_row in single_board.clone().iter_mut() {
                for single_input in single_board_row.iter_mut() {
                    if check_bingo(&single_board, board_size) {
                        println!("{:?}", single_board);
                        break;
                    } else {
                        if single_input.0 == num_to_mark {
                            single_input.1 = true;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn make_bingo_board_and_input(
    input: &mut std::str::Lines<'_>,
) -> (Vec<usize>, Vec<Vec<Vec<(usize, bool)>>>) {
    let mut bingo_boards: Vec<Vec<Vec<(usize, bool)>>> = Vec::new();

    let bingo_input: Vec<usize> = input
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    // while let line = input.next() {
    //     println!("{:?}", line);
    // }

    // for line in input.next() {
    //   println!("line :{}", line);
    // } println!("line :{}", line);

    let mut single_board: Vec<Vec<(usize, bool)>> = Vec::new();
    loop {
        let get_line = input.next();

        // println!("{:?}", get_line);

        match get_line {
            Some(single_line) => {
                if single_line.is_empty() {
                    continue;
                } else {
                    if single_board.len() < 4 {
                        // println!("single board: {:?}", single_board);
                        let get_single_board_one_line = single_line
                            .trim()
                            .split_ascii_whitespace()
                            .map(|i| (i.parse::<usize>().unwrap(), false))
                            .collect::<Vec<(usize, bool)>>();
                        single_board.push(get_single_board_one_line);
                    } else if single_board.len() == 4 {
                        let get_single_board_one_line = single_line
                            .trim()
                            .split_ascii_whitespace()
                            .map(|i| (i.parse::<usize>().unwrap(), false))
                            .collect::<Vec<(usize, bool)>>();
                        single_board.push(get_single_board_one_line);
                        bingo_boards.push(single_board);
                        single_board = Vec::new();
                    } else {
                        println!("Something strange happend 1");
                    }
                }
            }
            None => break,
        }
    }

    // println!("{:?}", input);

    (bingo_input, bingo_boards)
}

fn check_bingo(one_board: &Vec<Vec<(usize, bool)>>, board_size: usize) -> bool {
    // horizontal
    for i in 0..board_size {
        for j in 0..board_size {
            if one_board[i][j].1 == false {
                break;
            } else if one_board[i][j].1 == true && j == board_size {
                return true;
            }
        }
    }
    // vertical
    for i in 0..board_size {
        for j in 0..board_size {
            if one_board[j][i].1 == false {
                break;
            } else if one_board[j][i].1 == true && i == board_size {
                return true;
            }
        }
    }

    false
}

// [
//     [
//         [(22, false), (13, false), (17, false), (11, false), (0, false)],
//         [(8, false), (2, false), (23, false), (4,false), (24, false)],
//         [(21, false), (9, false), (14, false), (16, false), (7, false)],
//         [(6, false), (10, false), (3, false), (18, false), (5, false)],
//         [(1, false), (12, false), (20, false), (15, false), (19, false)]
//     ], [
//         [(3, false), (15,false), (0, false), (2, false), (22, false)],
//         [(9, false), (18, false), (13, false), (17, false), (5, false)],
//         [(19,false), (8, false), (7, false), (25, false), (23, false)],
//         [(20, false), (11, false), (10, false), (24, false), (4, false)],
//         [(14, false), (21, false), (16, false), (12, false), (6, false)]
//     ]
// ]
