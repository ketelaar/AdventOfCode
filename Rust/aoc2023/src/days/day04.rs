use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

pub fn solve() {
    let input = include_str!("../resources/day04.txt");

    let answer_a = get_answer_a(input);
    let answer_b = get_answer_b(input);

    println!("Answer for day 4 part 1 is: {answer_a}");
    println!("Answer for day 4 part 2 is: {answer_b}");
}

fn get_answer_a(input: &str) -> usize {
    let cards = get_cards(input);
    cards.iter().map(|c| c.get_score()).sum()
}

fn get_answer_b(input: &str) -> usize {
    let cards = get_cards(input);
    let maximum_card_number = cards.len();

    let mut card_number_map: HashMap<usize, usize> =
        cards.iter().map(|card| (card.number, 1)).collect();

    let mut card_counter = maximum_card_number.clone();

    for card in cards {
        dbg!(card.number);
        let number_of_cards = card_number_map.get(&card.number).unwrap().to_owned();
        let matching_numbers = card.get_matching_number_amount();

        let lower_bound = if card.number == maximum_card_number {
            card.number
        } else {
            card.number + 1
        };

        let upper_bound = card.number + matching_numbers;

        let extra_card_range = lower_bound..=upper_bound;
        increment_card_map_and_counter(
            &mut card_number_map,
            &mut card_counter,
            extra_card_range,
            number_of_cards,
            maximum_card_number,
        );
    }

    card_counter
}

fn increment_card_map_and_counter(
    map: &mut HashMap<usize, usize>,
    card_counter: &mut usize,
    range: RangeInclusive<usize>,
    increase_factor: usize,
    maximum_card_number: usize,
) {
    for number in range {
        if number >= maximum_card_number {
            return;
        }
        *card_counter += increase_factor;
        let original_value = map.get(&number).unwrap_or(&0);
        map.insert(number, original_value + increase_factor);
    }
}

fn get_cards(input: &str) -> Vec<Card> {
    let cards: Vec<Card> = input
        .lines()
        .filter_map(|l| {
            let (card_str, numbers_str) = l.split_once(':')?;
            let (_, card_number_str) = card_str.split_once(' ')?;

            Some(Card::parse(numbers_str, card_number_str))
        })
        .collect();
    cards
}

struct Card {
    number: usize,
    winning_numbers: HashSet<usize>,
    your_numbers: HashSet<usize>,
}

impl Card {
    fn parse(input: &str, card_number_str: &str) -> Card {
        let (winning_numbers_str, your_numbers_str) = input.split_once('|').unwrap();

        let winning_numbers = split_spaced_numbers(winning_numbers_str);
        let your_numbers = split_spaced_numbers(your_numbers_str);

        Card {
            number: card_number_str
                .trim()
                .parse()
                .expect("should be a valid number"),
            winning_numbers,
            your_numbers,
        }
    }

    fn get_matching_number_amount(&self) -> usize {
        self.winning_numbers
            .intersection(&self.your_numbers)
            .collect::<Vec<_>>()
            .len()
    }

    fn get_score(&self) -> usize {
        match self.get_matching_number_amount() {
            0 => 0,
            _ => 2_usize.pow((self.get_matching_number_amount() - 1) as u32),
        }
    }
}

fn split_spaced_numbers(input: &str) -> HashSet<usize> {
    input
        .trim()
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    const REAL_INPUT: &str = include_str!("../resources/day04.txt");

    #[test]
    fn example_a() {
        let expected_answer = 13;
        let actual_answer = get_answer_a(INPUT_EXAMPLE);

        assert_eq!(expected_answer, actual_answer);
    }

    #[test]
    fn answer_a() {
        let expected_answer = 21485;
        let actual_answer = get_answer_a(REAL_INPUT);

        assert_eq!(expected_answer, actual_answer);
    }

    #[test]
    fn example_b() {
        let expected_answer = 30;
        let actual_answer = get_answer_b(INPUT_EXAMPLE);

        assert_eq!(expected_answer, actual_answer);
    }
}
