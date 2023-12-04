use std::collections::HashSet;
use std::fs::read_to_string;

const FILENAME: &str = "input.txt";

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    copies: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let a: HashSet<u32> = HashSet::from_iter(self.winning_numbers.iter().cloned());
        let b: HashSet<u32> = HashSet::from_iter(self.numbers.iter().cloned());

        return a.intersection(&b).count() as u32;
    }
}

fn parse_card(s: &str) -> Card {
    let mut c = Card {
        id: 0,
        copies: 1,
        winning_numbers: Vec::new(),
        numbers: Vec::new(),
    };

    let mut split_line = s.split(":");

    c.id = split_line
        .next()
        .unwrap()
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .fold(0, |acc, elem| acc * 10 + elem);

    let mut nums = split_line.next().unwrap().split("|");

    c.winning_numbers = nums
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    c.numbers = nums
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    return c;
}

fn main() {
    let file = read_to_string(FILENAME).expect("unable to read file");

    let mut cards: Vec<Card> = Vec::new();
    for line in file.lines() {
        let card = parse_card(line);
        cards.push(card);
    }

    let cards_clone = cards.clone();
    for (idx, card) in cards_clone.iter().enumerate() {
        let copies = card.score();
        if copies == 0 {
            continue;
        }

        for i in 1..copies + 1 {
            cards[idx + i as usize].copies += cards[idx].copies;
        }
    }

    let mut num_cards_total = 0;
    for card in cards {
        num_cards_total += card.copies;
    }
    println!("{num_cards_total}");
}
