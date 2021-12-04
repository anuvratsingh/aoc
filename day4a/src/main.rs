use std::{fs::read_to_string, io::Result, path::Path};

fn main() -> Result<()> {
    let file = read_to_string(Path::new("test.txt")).unwrap(); // can't find a way to get txt as string blob from bufreader
                                                               // println!("{:?}", file.lines().next().unwrap());
                                                               // println!("{:?}", file.lines()[0]);
    let (bingo_input, bingo_boards) = make_bingo_board_and_input(&mut file.lines());
    // println!("{:?}, {:?}", bingo_input, bingo_boards);

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
    // }

    loop {
        let mut i = 0;
        let get_line = input.next();
        println!("{:?}", get_line);
        i = i + 1;
        if i > 10 {
            break;
        }
    }

    // println!("{:?}", input);

    (bingo_input, bingo_boards)
}
