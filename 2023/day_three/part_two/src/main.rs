use std::fs::read_to_string;

const FILENAME: &str = "input.txt";

const STAR: i32 = -1;
const EVERYTHING_ELSE: i32 = -2;

fn main() {
    let file = read_to_string(FILENAME).expect("unable to read file");

    // gear_candidates is a vector containing a tuple with the following structure:
    //  (part_number, (x_pos, y_pos))
    // where the (x_pos, y_pos) tuple is the position of the star the part number was by.
    let mut gear_candidates: Vec<(i32, (u32, u32))> = Vec::new();

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut height: u32 = 0;
    let mut width: u32 = 0;

    for line in file.lines() {
        width = line.len() as u32;

        let mut x: Vec<i32> = Vec::new();
        for ch in line.chars() {
            if ch == '*' {
                x.push(STAR);
                continue;
            }

            if ch.is_numeric() {
                x.push(ch.to_digit(10).unwrap() as i32);
                continue;
            }

            x.push(EVERYTHING_ELSE)
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
                    let mut star_pos: (u32, u32) = (0, 0);

                    // We're at the end of a number, check to see if its close to a symbol.
                    for num_coordinate in num_coordinates.iter() {
                        // west
                        if num_coordinate.0 != 0 {
                            let i = matrix
                                .get(num_coordinate.1 as usize)
                                .unwrap()
                                .get(num_coordinate.0 as usize - 1)
                                .unwrap();

                            if (*i) == STAR {
                                star_pos = (num_coordinate.0 - 1, num_coordinate.1);
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

                            if (*i) == STAR {
                                star_pos = (num_coordinate.0 + 1, num_coordinate.1);
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

                            if (*i) == STAR {
                                star_pos = (num_coordinate.0, num_coordinate.1 - 1);
                                break;
                            }

                            // northeast
                            if num_coordinate.0 < width - 1 {
                                let i = matrix
                                    .get(num_coordinate.1 as usize - 1)
                                    .unwrap()
                                    .get(num_coordinate.0 as usize + 1)
                                    .unwrap();

                                if (*i) == STAR {
                                    star_pos = (num_coordinate.0 + 1, num_coordinate.1 - 1);
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

                                if (*i) == STAR {
                                    star_pos = (num_coordinate.0 - 1, num_coordinate.1 - 1);
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

                            if (*i) == STAR {
                                star_pos = (num_coordinate.0, num_coordinate.1 + 1);
                                break;
                            }

                            // southeast
                            if num_coordinate.0 < width - 1 {
                                let i = matrix
                                    .get(num_coordinate.1 as usize + 1)
                                    .unwrap()
                                    .get(num_coordinate.0 as usize + 1)
                                    .unwrap();

                                if (*i) == STAR {
                                    star_pos = (num_coordinate.0 + 1, num_coordinate.1 + 1);
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

                                if (*i) == STAR {
                                    star_pos = (num_coordinate.0 - 1, num_coordinate.1 + 1);
                                    break;
                                }
                            }
                        }
                    }

                    if star_pos.0 != 0 || star_pos.1 != 0 {
                        gear_candidates.push((
                            num_coordinates
                                .iter()
                                .map(|elem| {
                                    matrix.get(elem.1 as usize).unwrap().get(elem.0 as usize)
                                })
                                .fold(0, |acc, elem| acc * 10 + elem.unwrap()),
                            (star_pos.0, star_pos.1),
                        ));
                    }

                    num_coordinates.clear();
                }
            }
        }
    }

    let mut sum_gear_ratios: u64 = 0;

    let gear_candidates_clone = gear_candidates.clone();
    let mut gear_candidates_iter = gear_candidates_clone.iter();
    let mut idx = 0;

    while let Some(gear_candidate) = gear_candidates_iter.next() {
        idx += 1;

        for potential_gear_match in gear_candidates.iter().skip(idx) {
            if gear_candidate.1 == potential_gear_match.1 {
                sum_gear_ratios += gear_candidate.0 as u64 * potential_gear_match.0 as u64
            }
        }
    }

    println!("{sum_gear_ratios}");
}
