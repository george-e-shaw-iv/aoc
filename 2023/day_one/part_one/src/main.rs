use std::fs::read_to_string;

const FILENAME: &str = "input.txt";

fn main() {
    let file = read_to_string(FILENAME).expect("unable to read file");
    let mut answer: u32 = 0;

    for line in file.lines() {
        let mut digits: Vec<u32> = line
            .to_string()
            .chars()
            .flat_map(|ch| ch.to_digit(10))
            .collect();

        if digits.len() > 2 {
            let _: Vec<u32> = digits.drain(1..digits.len() - 1).collect();
        } else {
            digits.resize(2, digits[0]);
        }

        let result: u32 = digits.iter().fold(0, |acc, elem| acc * 10 + elem);

        answer += result;
    }

    println!("{answer}");
}
