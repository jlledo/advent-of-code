use std::collections::HashSet;
use std::str::FromStr;

use winnow::ascii::{digit1, space1};
use winnow::combinator::{delimited, separated, separated_pair};
use winnow::{PResult, Parser};

fn main() -> color_eyre::eyre::Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .expect("first argument should be the input file");
    let input = std::fs::read_to_string(input_file)?;

    let points_sum = points_sum(&input);
    println!("Sum of points: {points_sum}");

    let card_count = card_count(&input);
    println!("Total scratchcards: {card_count}");

    Ok(())
}

fn points_sum(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Card::from_str(line).unwrap().points())
        .sum()
}

fn card_count(input: &str) -> u32 {
    let mut copies = vec![1; input.lines().count()];
    let mut card_count = 0;
    for line in input.lines() {
        let card: Card = line.parse().unwrap();
        let card_number = card.number as usize;
        let matching = card.matching_numbers_count();

        let copies_of_number = copies[card_number - 1];
        for i in card_number..card_number + matching {
            copies[i] += copies_of_number;
        }
        card_count += copies_of_number
    }

    card_count
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        let matching_numbers_count = self.matching_numbers_count();
        if matching_numbers_count > 0 {
            2u32.pow(matching_numbers_count as u32 - 1)
        } else {
            0
        }
    }

    fn matching_numbers_count(&self) -> usize {
        let winning_numbers: HashSet<&u32> = HashSet::from_iter(self.winning_numbers.iter());
        self.numbers_you_have
            .iter()
            .filter(|number| winning_numbers.contains(number))
            .count()
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_number, (winning_numbers, numbers_you_have)) = (
            delimited(("Card", space1), digit1.parse_to(), (':', space1)),
            separated_pair(separated_u32, (space1, '|', space1), separated_u32),
        )
            .parse(s)
            .map_err(|e| e.to_string())?;

        Ok(Card {
            number: card_number,
            winning_numbers,
            numbers_you_have,
        })
    }
}

fn separated_u32(input: &mut &str) -> PResult<Vec<u32>> {
    separated(1.., digit1.parse_to::<u32>(), space1).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_parsing() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(
            Card {
                number: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                numbers_you_have: vec![83, 86, 6, 31, 17, 9, 48, 53]
            },
            input.parse().unwrap()
        );
    }

    #[test]
    fn card_points() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, points_sum(input));
    }

    #[test]
    fn card_count() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, super::card_count(input));
    }
}
