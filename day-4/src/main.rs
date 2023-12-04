use std::{collections::HashSet, fs::read_to_string, path::Path};

use regex::{Regex, Match};

fn main() {
    let input = read_to_string(Path::new(
        "/Users/darioalessandro/Documents/advent-of-code-2023/day-4/src/input.txt",
    )).unwrap();
    let actual_scores = rate_cards(&input);
    println!("score {}", actual_scores.iter().sum::<u32>());
}

#[derive(Default)]
struct Card <'a> {
    id: u32,
    winning_numbers: Vec<Match<'a>>,
    numbers: Vec<Match<'a>>
}

fn parse_card(input: &str) -> Card {
    println!("input {}", input);
    let number_regex = Regex::new(r"([0-9|])+").unwrap();
    let mut all_characters_needed = number_regex.find_iter(input);
    let id:u32 = all_characters_needed.next().unwrap().as_str().parse().unwrap();
    // winning numbers are all the numbers to the left of the | symbol
    
    let mut found_bar = false;
    let (winning_numbers, numbers ): (Vec<Match>, Vec<Match>) = all_characters_needed.fold((Vec::new(), Vec::new()), |(mut winning_numbers, mut numbers), found_charcter| {
            if found_charcter.as_str() == "|" {
                found_bar = true;
            } 
            if found_bar {
                numbers.push(found_charcter);
            } else {
                winning_numbers.push(found_charcter)
            }
            (winning_numbers, numbers)
    });
    Card {
        id,
        winning_numbers,
        numbers
    }
}

fn rate_cards(input: &str) -> Vec<u32> {
    // 1 parse cards
    let cards: Vec<Card> = input.split('\n').map(parse_card).collect();
    // score each card.
    cards.iter().map(|card| {
        let Card{ id, winning_numbers, numbers} = card;
        // find all the winning_number in numbers;
        let mut score = 0;
        for number in numbers {
            if winning_numbers.iter().find(|winning| winning.as_str() == number.as_str()).is_some() {
                if score == 0 {
                    score = 1;
                } else {
                    score = score * 2;
                }
            }
        }
        score
    }).collect()
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

        let expected_scores = vec![8, 2, 2, 1, 0, 0];
        let expected_total: u32 = 13;

        let actual_scores = rate_cards(&input);
        assert_eq!(expected_scores, actual_scores);
        assert_eq!(expected_total, actual_scores.iter().sum())

    }
}