use std::fs::read_to_string;

const FILENAME: &str = "input.txt";

const SYMBOL: i32 = -1;
const PERIOD: i32 = -2;

fn main() {
    let file = read_to_string(FILENAME).expect("unable to read file");

    let mut part_numbers: Vec<i32> = Vec::new();

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut height: u32 = 0;
    let mut width: u32 = 0;

    for line in file.lines() {
        width = line.len() as u32;

        let mut x: Vec<i32> = Vec::new();
        for ch in line.chars() {
            if ch == '.' {
                x.push(PERIOD);
                continue;
            }

            if ch.is_numeric() {
                x.push(ch.to_digit(10).unwrap() as i32);
                continue;
            }

            x.push(SYMBOL)
        }

        height += 1;
        matrix.push(x);
    }

    // for row in matrix.iter() {
    //     for col in row.iter() {
    //         if (*col) == PERIOD {
    //             print!(".");
    //         } else if (*col) == SYMBOL {
    //             print!("!");
    //         } else {
    //             print!("{}", *col);
    //         }
    //     }
    //     print!("\n");
    // }

    let mut num_coordinates: Vec<(u32, u32)> = Vec::new();

    for (y_idx, y) in matrix.iter().enumerate() {
        for (x_idx, x) in y.iter().enumerate() {
            if (*x) >= 0 {
                num_coordinates.push((x_idx as u32, y_idx as u32));
            } else {
                if num_coordinates.len() > 0 {
                    let mut is_part_number: bool = false;

                    // We're at the end of a number, check to see if its close to a symbol.
                    for num_coordinate in num_coordinates.iter() {
                        // west
                        if num_coordinate.0 != 0 {
                            let i = matrix
                                .get(num_coordinate.1 as usize)
                                .unwrap()
                                .get(num_coordinate.0 as usize - 1)
                                .unwrap();

                            if (*i) == SYMBOL {
                                is_part_number = true;
                                break;
                            }
                        }

                        // east
                        if num_coordinate.0 < width - 1 {
                            let i = matrix
                                .get(num_coordinate.1 as usize)
                                .unwrap()
                                .get(num_coordinate.0 as usize + 1)
                                .unwrap();

                            if (*i) == SYMBOL {
                                is_part_number = true;
                                break;
                            }
                        }

                        // north
                        if num_coordinate.1 != 0 {
                            let i = matrix
                                .get(num_coordinate.1 as usize - 1)
                                .unwrap()
                                .get(num_coordinate.0 as usize)
                                .unwrap();

                            if (*i) == SYMBOL {
                                is_part_number = true;
                                break;
                            }

                            // northeast
                            if num_coordinate.0 < width - 1 {
                                let i = matrix
                                    .get(num_coordinate.1 as usize - 1)
                                    .unwrap()
                                    .get(num_coordinate.0 as usize + 1)
                                    .unwrap();

                                if (*i) == SYMBOL {
                                    is_part_number = true;
                                    break;
                                }
                            }

                            // northwest
                            if num_coordinate.0 != 0 {
                                let i = matrix
                                    .get(num_coordinate.1 as usize - 1)
                                    .unwrap()
                                    .get(num_coordinate.0 as usize - 1)
                                    .unwrap();

                                if (*i) == SYMBOL {
                                    is_part_number = true;
                                    break;
                                }
                            }
                        }

                        // south
                        if num_coordinate.1 < height - 1 {
                            let i = matrix
                                .get(num_coordinate.1 as usize + 1)
                                .unwrap()
                                .get(num_coordinate.0 as usize)
                                .unwrap();

                            if (*i) == SYMBOL {
                                is_part_number = true;
                                break;
                            }

                            // southeast
                            if num_coordinate.0 < width - 1 {
                                let i = matrix
                                    .get(num_coordinate.1 as usize + 1)
                                    .unwrap()
                                    .get(num_coordinate.0 as usize + 1)
                                    .unwrap();

                                if (*i) == SYMBOL {
                                    is_part_number = true;
                                    break;
                                }
                            }

                            // southwest
                            if num_coordinate.0 != 0 {
                                let i = matrix
                                    .get(num_coordinate.1 as usize + 1)
                                    .unwrap()
                                    .get(num_coordinate.0 as usize - 1)
                                    .unwrap();

                                if (*i) == SYMBOL {
                                    is_part_number = true;
                                    break;
                                }
                            }
                        }
                    }

                    if is_part_number {
                        part_numbers.push(
                            num_coordinates
                                .iter()
                                .map(|elem| {
                                    matrix.get(elem.1 as usize).unwrap().get(elem.0 as usize)
                                })
                                .fold(0, |acc, elem| acc * 10 + elem.unwrap()),
                        );
                    }

                    num_coordinates.clear();
                }
            }
        }
    }

    let mut sum_part_numbers: i32 = 0;
    for part_number in part_numbers {
        sum_part_numbers += part_number
    }

    println!("{sum_part_numbers}");
}
