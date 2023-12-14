use std::collections::HashSet;
use std::str::FromStr;

use winnow::ascii::{digit1, space1};
use winnow::combinator::{preceded, separated, separated_pair};
use winnow::{PResult, Parser};

fn main() -> color_eyre::eyre::Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .expect("first argument should be the input file");
    let input = std::fs::read_to_string(input_file)?;

    let points_sum = points_sum(&input);
    println!("Sum of points: {points_sum}");

    Ok(())
}

fn points_sum(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Card::from_str(line).unwrap().points())
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        let winning_numbers: HashSet<&u32> = HashSet::from_iter(self.winning_numbers.iter());
        let matching_numbers_count = self
            .numbers_you_have
            .iter()
            .filter(|number| winning_numbers.contains(number))
            .count();

        if matching_numbers_count > 0 {
            2u32.pow(matching_numbers_count as u32 - 1)
        } else {
            0
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (winning_numbers, numbers_you_have) = preceded(
            ("Card", space1, digit1, ':', space1),
            separated_pair(separated_u32, (space1, '|', space1), separated_u32),
        )
        .parse(s)
        .map_err(|e| e.to_string())?;

        Ok(Card {
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
}
