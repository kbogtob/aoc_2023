use std::collections::HashSet;

use regex::Regex;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    actual_numbers: HashSet<u32>,
}

fn parse_number(number_list: &str) -> HashSet<u32> {
    let number_parsing_re = Regex::new(r"(\d+)").unwrap();

    number_parsing_re
        .captures_iter(number_list)
        .map(|capture| capture.get(0).unwrap())
        .map(|number| {
            number
                .as_str()
                .parse::<u32>()
                .expect("Number should be a number")
        })
        .collect::<HashSet<u32>>()
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let card_info = line.split(":").collect::<Vec<&str>>();
        let card_numbers = *card_info.get(1).expect("Card should have numbers");

        let numbers = card_numbers.split("|").collect::<Vec<&str>>();
        let winning_numbers = *numbers.get(0).expect("Card should have winning numbers");
        let winning_numbers = parse_number(winning_numbers);

        let actual_numbers = *numbers.get(1).expect("Card should have actual numbers");
        let actual_numbers = parse_number(actual_numbers);

        Card {
            winning_numbers,
            actual_numbers,
        }
    }
}

impl Card {
    fn matching_numbers(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.actual_numbers)
            .count() as u32
    }

    fn points(&self) -> u32 {
        let intersection_size = self.matching_numbers();

        if intersection_size == 0 {
            0
        } else {
            2u32.pow(intersection_size - 1)
        }
    }
}

fn ex1(input: &str) -> u32 {
    input.lines().map(|line| Card::from(line).points()).sum()
}

fn ex2(input: &str) -> u32 {
    let mut total_scratch_cards = 0;

    let cards = input
        .lines()
        .map(|line| Card::from(line))
        .collect::<Vec<Card>>();

    let mut multipliers = vec![1; cards.len()];

    for (card_id, card) in cards.iter().enumerate() {
        let current_card_multiplier = multipliers[card_id];
        total_scratch_cards += current_card_multiplier;

        let bonus_card_range = card.matching_numbers() as usize;
        for affected_card_index in card_id + 1..=(card_id + bonus_card_range) {
            multipliers[affected_card_index] += current_card_multiplier
        }
    }

    total_scratch_cards
}

fn main() {
    let input = include_str!("../etc/input");

    println!("{}", ex1(input));
    println!("{}", ex2(input));
}
