use std::{fs::read_to_string, io::Result, path::Path};

fn main() -> Result<()> {
    let file = read_to_string(Path::new("test.txt")).unwrap(); // can't find a way to get txt as string blob from bufreader
                                                               // println!("{:?}", file.lines().next().unwrap());
                                                               // println!("{:?}", file.lines()[0]);
    let (bingo_input, bingo_boards) = make_bingo_board_and_input(&mut file.lines());

    // println!("Input: {:?}", bingo_boards);

    // for num_to_mark in bingo_input {
    //     for single_board in bingo_boards.iter_mut() {
    //         for single_board_row in single_board.iter_mut() {
    //             for single_input in single_board_row.iter_mut() {
    //                 if check_bingo(*single_board) {
    //                     println!("{:?}", single_board);
    //                     break;
    //                 } else {
    //                     if single_input.0 == num_to_mark {
    //                         single_input.1 = true;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // let mut bingo_boards_clone = bingo_boards.clone();

    // let result = bingo_input.iter().map(|num_to_mark| {
    //     bingo_boards_clone.iter_mut().map(|single_board| {
    //         let marked_num = mark_number(single_board, num_to_mark);
    //         if marked_num {
    //             if check_bingo(&single_board) {
    //                 sum_of_all_unmarked_numbers(&single_board) * num_to_mark.clone();
    //             }
    //         }
    //     })
    // });

    let _result = do_thing(bingo_input, bingo_boards);

    // println!("Result: {:?}", result);

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

fn check_bingo(one_board: &Vec<Vec<(usize, bool)>>) -> bool {
    // horizontal
    // println!("Cehck Bingo");
    for i in 0..5 {
        for j in 0..5 {
            if one_board[i][j].1 == false {
                break;
            } else if one_board[i][j].1 == true && j == 4 {
                println!("Wining board Row: {:?}", one_board);
                return true;
            }
        }
    }
    // vertical
    for i in 0..5 {
        for j in 0..5 {
            if one_board[j][i].1 == false {
                break;
            } else if one_board[j][i].1 == true && j == 4 {
                println!("Wining board Column: {:?}", one_board);
                return true;
            }
        }
     }
    println!("Not true");
    return false;
}

fn mark_number(single_board: &mut Vec<Vec<(usize, bool)>>, num_to_mark: usize) -> bool {
    // let inner_num_to_mark = num_to_mark.clone();
    for i in 0..5 {
        for j in 0..5 {
            if single_board[i][j].0 == num_to_mark {
                // println!("Before: {}", single_board[i][j].1);
                single_board[i][j].1 = true;
                // println!("bool: {}", single_board[i][j].1);
                // println!("value: {}", single_board[i][j].0);
                // println!("After: {:?}", single_board);
                return true;
            }
        }
    }
    // println!("Mark Number {:?}", single_board);
    return false;
}
fn sum_of_all_unmarked_numbers(single_board: &Vec<Vec<(usize, bool)>>) -> usize {
    let mut result = 0;
    for i in 0..5 {
        for j in 0..5 {
            if single_board[i][j].1 == false {
                result = result + single_board[i][j].0;
            }
        }
    }
    result
}

fn do_thing(bingo_input: Vec<usize>, bingo_boards: Vec<Vec<Vec<(usize, bool)>>>) -> usize {
    // let mut no_loop_one = 0;
    // let mut no_loop_two = 0;

    for num_to_mark in bingo_input.iter() {
        for single_board in bingo_boards.clone().iter_mut() {
            let marked_num = mark_number(single_board, *num_to_mark);
            // print!("In main loop single board{:?}", single_board);
            if marked_num {
                // println!("Here");
                if check_bingo(single_board) {
                    println!("Is this the problem");
                    return sum_of_all_unmarked_numbers(single_board) * num_to_mark;
                    // println!("Result in loop:{}", result);
                }
            }
            // no_loop_two = no_loop_two + 1;
            // println!("Two: {}", no_loop_two);
        }
        // no_loop_one = no_loop_one + 1;
        // println!("One: {}", no_loop_one);
    }
    panic!("It doesn't work without it");
}
