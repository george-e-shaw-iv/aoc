use std::fs::read_to_string;

const FILENAME: &str = "input.txt";

fn string_to_number(s: &str) -> u32 {
    match s {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    }
}

fn main() {
    let file = read_to_string(FILENAME).expect("unable to read file");
    let mut answer: u32 = 0;

    for line in file.lines() {
        // idx_number is a vector of tuples that contain the index and the number (either the string or numeric
        // representation, but both as type string) of each found number in the line.
        let mut idx_number: Vec<(usize, &str)> = Vec::new();

        idx_number.append(&mut line.match_indices(char::is_numeric).collect());
        idx_number.append(&mut line.match_indices("one").collect());
        idx_number.append(&mut line.match_indices("two").collect());
        idx_number.append(&mut line.match_indices("three").collect());
        idx_number.append(&mut line.match_indices("four").collect());
        idx_number.append(&mut line.match_indices("five").collect());
        idx_number.append(&mut line.match_indices("six").collect());
        idx_number.append(&mut line.match_indices("seven").collect());
        idx_number.append(&mut line.match_indices("eight").collect());
        idx_number.append(&mut line.match_indices("nine").collect());

        // Sort these by index (first element in tuple) to put them in order of where each was found.
        idx_number.sort_by(|a, b| a.0.cmp(&b.0));

        // Either drain the middle numbers or add a second number depending on the amount of numbers found in the
        // the string.
        if idx_number.len() > 2 {
            let _: Vec<_> = idx_number.drain(1..idx_number.len() - 1).collect();
        } else {
            idx_number.resize(2, idx_number[0]);
        }

        let digits: Vec<u32> = idx_number
            .iter()
            .map(|elem| string_to_number(elem.1))
            .collect();

        let result: u32 = digits.iter().fold(0, |acc, elem| acc * 10 + elem);
        answer += result;
    }

    println!("{answer}");
}
