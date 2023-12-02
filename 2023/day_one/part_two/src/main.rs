use std::fs::read_to_string;

const FILENAME: &str = "input.txt";

fn main() {
    let file = read_to_string(FILENAME).expect("unable to read file");
    let mut answer: u32 = 0;

    for line in file.lines() {
        let mut line = line.to_string();
        // oneight // <- fine, should be 1, will be 1
        // twone // <- not fine, should be 2, will be 1
        // threeight // <- fine, should be 3, will be 3
        // fiveight // <- fine, should be 5, will be 5
        // sevenine // <- fine, should be 7, will be 7
        // eightwo // <- not fine, will be 2, should be 8
        // eighthree // <- not fine, will be 3, should be 8
        // nineight // <- not fine, will be 8, should be 9

        if let Some(idx) = line.find("twone") {
            line.replace_range(idx..idx + "two".len(), "2");
        }

        if let Some(idx) = line.find("eightwo") {
            line.replace_range(idx..idx + "eight".len(), "8");
        }

        if let Some(idx) = line.find("eighthree") {
            line.replace_range(idx..idx + "eight".len(), "8");
        }

        if let Some(idx) = line.find("nineight") {
            line.replace_range(idx..idx + "nine".len(), "9");
        }

        line = line
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");

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
