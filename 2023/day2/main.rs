use std::str::FromStr;

fn main() -> color_eyre::eyre::Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .expect("first argument should be the input file");
    let input = std::fs::read_to_string(input_file)?;

    let sum = possible_game_ids_sum(&input);
    println!("Sum of possible game IDs: {sum}");

    Ok(())
}

fn possible_game_ids_sum(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .filter(game_is_possible)
        .map(|g| g.id)
        .sum()
}

fn game_is_possible(game: &Game) -> bool {
    game.subsets
        .iter()
        .all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Game {
    pub id: u32,
    pub subsets: Vec<Set>,
}

impl FromStr for Game {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let game = parts.next().unwrap();
        let id: u32 = game
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let subsets: Vec<Set> = parts
            .next()
            .unwrap()
            .split("; ")
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Game { id, subsets })
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl FromStr for Set {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set::default();
        for frequency in s.split(", ") {
            let mut parts = frequency.split_ascii_whitespace();
            let frequency: u32 = parts.next().unwrap().parse().unwrap();
            match parts.next().unwrap() {
                "red" => set.red += frequency,
                "green" => set.green += frequency,
                "blue" => set.blue += frequency,
                _ => panic!(),
            };
        }

        Ok(set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_from_str() {
        let string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game {
            id: 1,
            subsets: vec![
                Set {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Set {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Set {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };
        assert_eq!(game, string.parse().unwrap());
    }

    #[test]
    fn possible_game_ids_sum_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8, possible_game_ids_sum(input));
    }
}
