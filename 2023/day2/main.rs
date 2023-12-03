use std::str::FromStr;

fn main() -> color_eyre::eyre::Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .expect("first argument should be the input file");
    let input = std::fs::read_to_string(input_file)?;

    let games: Vec<Game> = input.lines().map(|line| line.parse().unwrap()).collect();
    let possible_game_ids_sum: u32 = games
        .iter()
        .filter(|game| {
            game.is_possible_with(Set {
                red: 12,
                green: 13,
                blue: 14,
            })
        })
        .map(|game| game.id)
        .sum();
    println!("Sum of possible game IDs: {possible_game_ids_sum}");

    let minimum_set_power_sum: u32 = games.iter().map(|game| game.minimum_set().power()).sum();
    println!("Sum of the power of minimum sets: {minimum_set_power_sum}");

    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Game {
    pub id: u32,
    pub subsets: Vec<Set>,
}

impl Game {
    fn is_possible_with(&self, set: Set) -> bool {
        self.subsets.iter().all(|subset| set.is_superset(subset))
    }

    fn minimum_set(&self) -> Set {
        self.subsets
            .iter()
            .fold(Set::default(), |mut minimum_set, set| {
                minimum_set.red = std::cmp::max(minimum_set.red, set.red);
                minimum_set.green = std::cmp::max(minimum_set.green, set.green);
                minimum_set.blue = std::cmp::max(minimum_set.blue, set.blue);
                minimum_set
            })
    }
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Set {
    fn is_superset(&self, other: &Set) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
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
}
