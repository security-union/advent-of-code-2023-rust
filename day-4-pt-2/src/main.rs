use std::{collections::HashSet, fs::read_to_string, path::Path};

use regex::{Match, Regex};

fn main() {
    let input = read_to_string(Path::new(
        "/Users/darioalessandro/Documents/advent-of-code-2023/day-4/src/input.txt",
    ))
    .unwrap();
    let cards: Vec<Card> = input.split('\n').map(parse_card).collect();
    let actual_sum = find_total_cards(&cards);
    println!("actual sum {}", actual_sum);
}

#[derive(Default, Debug)]
struct Card<'a> {
    id: u32,
    winning_numbers: Vec<Match<'a>>,
    numbers: Vec<Match<'a>>,
    score: u32,
}

fn parse_card(input: &str) -> Card {
    println!("input {}", input);
    let number_regex = Regex::new(r"([0-9|])+").unwrap();
    let mut all_characters_needed = number_regex.find_iter(input);
    let id: u32 = all_characters_needed
        .next()
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    let mut found_bar = false;
    let (winning_numbers, numbers): (Vec<Match>, Vec<Match>) = all_characters_needed.fold(
        (Vec::new(), Vec::new()),
        |(mut winning_numbers, mut numbers), found_charcter| {
            if found_charcter.as_str() == "|" {
                found_bar = true;
            }
            if found_bar {
                numbers.push(found_charcter);
            } else {
                winning_numbers.push(found_charcter)
            }
            (winning_numbers, numbers)
        },
    );
    let mut score = 0;
    for number in &numbers {
        if winning_numbers
            .iter()
            .find(|winning| winning.as_str() == number.as_str())
            .is_some()
        {
            score = score + 1;
        }
    }
    Card {
        id,
        winning_numbers,
        numbers,
        score,
    }
}

fn find_cards(cards: &[Card], i: usize) -> u32 {
    cards.get(i).map_or(0, |card| {
        let score = card.score as usize;
        let start = i + 1;
        (1..=score).fold(1, |acc, j| acc + find_cards(cards, start + j - 1))
    })
}

fn find_total_cards(cards: &[Card]) -> u32 {
    cards
        .iter()
        .enumerate()
        .fold(0, |acc, (i, _card)| acc + find_cards(cards, i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_cards() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let expected_total_cards = 30;
        let cards: Vec<Card> = input.split('\n').map(parse_card).collect();
        let actual_sum = find_total_cards(&cards);
        assert_eq!(expected_total_cards, actual_sum);
    }
}
