use std::fs::read_to_string;

const FILENAME: &str = "input.txt";

#[derive(Copy, Clone, Debug)]
struct Pull(u32, u32, u32); // r,g,b

#[derive(Debug)]
struct Game(u32, Vec<Pull>); // id, pulls

impl Game {
    fn min_colors_possible(&self) -> Pull {
        let mut min_possible = Pull(0, 0, 0);
        for pull in self.1.as_slice() {
            if pull.0 > min_possible.0 {
                min_possible.0 = pull.0
            }

            if pull.1 > min_possible.1 {
                min_possible.1 = pull.1
            }

            if pull.2 > min_possible.2 {
                min_possible.2 = pull.2
            }
        }
        return min_possible;
    }
}

impl Pull {
    fn power(&self) -> u32 {
        self.0 * self.1 * self.2
    }
}

// Parse the following line format:
//  Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// into a Game struct.
fn parse_game(line: &str) -> Game {
    // This should result in a vector with two elements:
    //  ["Game 1", "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]
    let mut split_line = line.split(":");

    // Take the first element and extract the game id.
    let game_id = split_line
        .next()
        .unwrap()
        .chars()
        .filter_map(|elem| elem.to_digit(10))
        .fold(0, |acc, elem| acc * 10 + elem);

    // Split the second element into individual pulls of the colors, which would result in:
    //  ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
    let pulls: Vec<&str> = split_line
        .next()
        .unwrap()
        .split(";")
        .map(|elem| elem.trim())
        .collect();

    let mut parsed_pulls: Vec<Pull> = Vec::new();
    for pull in pulls {
        let mut current_pull = Pull(0, 0, 0);

        // Split an individual pull into the number/colors:
        //  ["3 blue", "4 red"]
        // Iterate over each and extract the number and color to set the values in current_pull.
        for number_and_color in pull.split(",").map(|elem| elem.trim()) {
            let mut split = number_and_color.split_whitespace();

            let number = split.next().unwrap().trim();
            let color = split.next().unwrap().trim();

            match color {
                "red" => current_pull.0 = number.parse::<u32>().unwrap(),
                "green" => current_pull.1 = number.parse::<u32>().unwrap(),
                "blue" => current_pull.2 = number.parse::<u32>().unwrap(),
                _ => println!("this shouldn't happen but i don't know how to do exception handling yet. got color {}", color)
            }
        }

        parsed_pulls.push(current_pull)
    }

    return Game(game_id, parsed_pulls);
}

fn main() {
    let file = read_to_string(FILENAME).expect("unable to read file");
    let mut summation_of_powers: u32 = 0;

    for line in file.lines() {
        let game = parse_game(line);
        summation_of_powers += game.min_colors_possible().power()
    }

    println!("{summation_of_powers}")
}
