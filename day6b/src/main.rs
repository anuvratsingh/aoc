use std::{fs::read_to_string, path::Path};
fn main() {
    let file = read_to_string(Path::new("test.txt")).unwrap();
    // let file = read_to_string(Path::new("input.txt")).unwrap();

    let cycle = 2;

    let parsed_input = new_new_parser(&mut file.lines());

    let _result = new_new_reproducer(parsed_input, cycle);

    // println!("{:?}", parsed_input);
    // println!("Parsed Input: {:?}", result);
}

// [[0, 0], [1, 1], [2, 1], [3, 2], [4, 1], [5, 0], [6, 0], [7, 0], [8, 0]]

fn new_new_reproducer(input: Vec<Vec<usize>>, cycles: usize) -> Vec<Vec<usize>> {
    let mut output = input.clone();

    for _ in 0..cycles {
        for i in &mut output {
            if i[1] > 0 {
                i
            } 
        }
    }
    output
}
fn new_new_parser(input: &mut std::str::Lines<'_>) -> Vec<Vec<usize>> {
    let array: Vec<usize> = input
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut output: Vec<Vec<usize>> = vec![
        vec![0, 0],
        vec![1, 0],
        vec![2, 0],
        vec![3, 0],
        vec![4, 0],
        vec![5, 0],
        vec![6, 0],
        vec![7, 0],
        vec![8, 0],
    ];

    for i in array {
        for j in 0..output.len() {
            if output[j][0] == i {
                output[j][1] = output[j][1] + 1;
            }
        }
    }

    output
}
// fn _new_reproduce(input: &HashMap<i8, usize>, duration: usize) -> HashMap<i8, usize> {
//     let mut output: HashMap<i8, usize> = input
//         .iter()
//         .map(|(&age, &count)| (age - 1, count))
//         .collect();

//     // for i in 0..duration {
//     //     // let _add: HashMap<i8, usize> = output
//     //     //     .iter()
//     //     //     .map(|(&mut age, &mut count)| {
//     //     //         if age != 0 {
//     //     //             (age, count)
//     //     //         } else {
//     //     //             output.insert(8, count);
//     //     //             (6, count)
//     //     //         }
//     //     //     })
//     //     //     .collect::<HashMap<i8, usize>>();
//     //     // if output.contains_key(&-1) {
//     //     //     println!("Output in loop: {:?}", output);
//     //     //     let rebirth_for_fish = output.remove(&-1).unwrap();
//     //     //     output.insert(6, rebirth_for_fish);
//     //     //     output.insert(8, rebirth_for_fish);
//     //     // }

//     //     // let _subtract: HashMap<i8, usize> = output
//     //     //     .iter()
//     //     //     .map(|(&age, &count)| (age - 1, count))
//     //     //     .collect();
//     //     // let test = output.clone();
//     //     // for (age, _) in &test {
//     //     //     if *age == 0 {
//     //     //         let removed = output.remove(&0).unwrap();
//     //     //         output.insert(6, removed);
//     //     //         output.insert(8, removed);
//     //     //     } else {
//     //     //         let removed = output.remove(&age).unwrap();
//     //     //         output.insert(age - 1, removed);
//     //     //     }
//     //     //     println!("iter: {} output: {:?}", i, output);
//     //     //     println!("iter: {} test: {:?}", i, test);
//     //     // }
//     // }

//     output
// }
// fn _new_parser(input: &mut std::str::Lines<'_>) -> HashMap<i8, usize> {
//     let array = input
//         .next()
//         .unwrap()
//         .trim()
//         .split(',')
//         .map(|v| v.parse::<i8>().unwrap())
//         .collect::<Vec<i8>>();

//     let mut output = HashMap::new();

//     for i in array {
//         if output.contains_key(&i) {
//             *output.get_mut(&i).unwrap() += 1;
//         } else {
//             output.insert(i, 1);
//         }
//     }
//     output
// }

// fn _reproduce(input: &mut Vec<usize>, duration: usize) -> &mut Vec<usize> {
//     for _ in 0..duration {
//         for i in 0..input.len() {
//             if input[i] != 0 {
//                 input[i] = input[i] - 1;
//             } else if input[i] == 0 {
//                 input.push(8);
//                 input[i] = 6;
//             }
//         }
//     }
//     input
// }
// fn _parse_input(input: &mut std::str::Lines<'_>) -> Vec<usize> {
//     let output = input
//         .next()
//         .unwrap()
//         .trim()
//         .split(",")
//         .map(|x| x.parse::<usize>().unwrap())
//         .collect();
//     // loop {
//     //     let next_line = input.next();

//     //     match next_line {
//     //         Some(line) => {
//     //             output = line
//     //                 .trim()
//     //                 .split(",")
//     //                 .map(|x| x.parse::<usize>().unwrap())
//     //                 .collect();
//     //         }
//     //         None => break,
//     //     }
//     // }

//     output
// }


// fn new_new_reproducer(input: Vec<Vec<usize>>, cycles: usize) -> Vec<Vec<usize>> {
//     let mut output = input.clone();

//     for l in 0..cycles {
//         // println!("Output BEFORE iter {}:{:?}", l, output);
//         for i in 0..output.len() {
//             if i != 0 {
//                 if output[i][1] != 0 {
//                     output[i - 1][1] = output[i - 1][1] + output[i][1];
//                     output[i][1] = output[i][1] - output[i][1];
//                 }
//             } else {
//                 if output[0][1] > 0 {
//                     println!("Before Inside i=0,Iter{}, output is{:?}", l, output);
//                     output[6][1] = output[6][1] + output[0][1];
//                     output[8][1] = output[8][1] + output[0][1];
//                     output[0][1] = output[0][1] - output[0][1];
//                     println!("After Inside i=0,Iter{}, output is{:?}", l, output);
//                     break;
//                 }
//             }
//         }
//         println!("Output AFTER iter {}:{:?}", l, output);
//     }
//     output
// }
