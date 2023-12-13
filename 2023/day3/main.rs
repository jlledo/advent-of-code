fn main() -> color_eyre::eyre::Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .expect("first argument should be the input file");
    let input = std::fs::read_to_string(input_file)?;

    let parts = parts(&input);
    let part_numbers_sum: u32 = parts.iter().flat_map(|part| &part.numbers).sum();
    println!("Sum of part numbers: {part_numbers_sum}");

    let gear_ratios_sum: u64 = parts
        .iter()
        .filter_map(|part| part.try_into().ok())
        .map(|gear: Gear| gear.ratio())
        .sum();
    println!("Sum of gear ratios: {gear_ratios_sum}");

    Ok(())
}

fn parts<'a>(input: &'a str) -> Vec<Part> {
    let mut parts = vec![];
    let lines: Vec<&'a str> = input.lines().collect();
    for (row, line) in lines.iter().enumerate() {
        for (column, c) in line.as_bytes().iter().enumerate() {
            if is_part(*c) {
                parts.push(Part {
                    character: *c as char,
                    numbers: part_numbers(&lines, row, column),
                });
            }
        }
    }

    parts
}

fn is_part(c: u8) -> bool {
    c != b'.' && !c.is_ascii_digit()
}

fn part_numbers<'a>(lines: &Vec<&'a str>, row: usize, column: usize) -> Vec<u32> {
    let mut part_numbers = vec![];
    if row > 0 {
        if let Some(top) = scan_number(lines, row - 1, column) {
            part_numbers.push(top);
        } else {
            if column > 0 {
                if let Some(top_left) = scan_number(lines, row - 1, column - 1) {
                    part_numbers.push(top_left);
                }
            }
            if column < lines[row].len() - 1 {
                if let Some(top_right) = scan_number(lines, row - 1, column + 1) {
                    part_numbers.push(top_right);
                }
            }
        }
    }

    if column > 0 {
        if let Some(left) = scan_number(lines, row, column - 1) {
            part_numbers.push(left);
        }
    }

    if column < lines[row].len() - 1 {
        if let Some(right) = scan_number(lines, row, column + 1) {
            part_numbers.push(right);
        }
    }

    if row < lines.len() - 1 {
        if let Some(bottom) = scan_number(lines, row + 1, column) {
            part_numbers.push(bottom);
        } else {
            if column > 0 {
                if let Some(bottom_left) = scan_number(lines, row + 1, column - 1) {
                    part_numbers.push(bottom_left);
                }
            }
            if column < lines[row].len() - 1 {
                if let Some(bottom_right) = scan_number(lines, row + 1, column + 1) {
                    part_numbers.push(bottom_right);
                }
            }
        }
    }

    part_numbers
}

fn scan_number<'a>(lines: &Vec<&'a str>, row: usize, column: usize) -> Option<u32> {
    if !lines[row].as_bytes()[column].is_ascii_digit() {
        return None;
    }

    let mut left = column;
    let mut right = left;
    let line = lines[row];
    while left > 0 {
        if !line.as_bytes()[left - 1].is_ascii_digit() {
            break;
        }
        left -= 1;
    }
    while right < line.len() - 1 {
        if !line.as_bytes()[right + 1].is_ascii_digit() {
            break;
        }
        right += 1;
    }

    line[left..=right].parse().ok()
}

struct Part {
    pub character: char,
    pub numbers: Vec<u32>,
}

struct Gear {
    numbers: (u32, u32),
}

impl Gear {
    fn ratio(&self) -> u64 {
        self.numbers.0 as u64 * self.numbers.1 as u64
    }
}

struct Error;

impl TryFrom<Part> for Gear {
    type Error = Error;

    fn try_from(value: Part) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Part> for Gear {
    type Error = Error;

    fn try_from(value: &Part) -> Result<Self, Self::Error> {
        if value.character == '*' && value.numbers.len() == 2 {
            Ok(Self {
                numbers: (value.numbers[0], value.numbers[1]),
            })
        } else {
            Err(Error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0, "...\n.#.\n..."; "part with no numbers returns zero")]
    #[test_case(1, "1..\n.#.\n..."; "part with top-left number returns number")]
    #[test_case(1, ".1.\n.#.\n..."; "part with top number returns number")]
    #[test_case(1, "..1\n.#.\n..."; "part with top-right number returns number")]
    #[test_case(1, "...\n1#.\n..."; "part with left number returns number")]
    #[test_case(1, "...\n.#1\n..."; "part with right number returns number")]
    #[test_case(1, "...\n.#.\n1.."; "part with bottom-left number returns number")]
    #[test_case(1, "...\n.#.\n.1."; "part with bottom number returns number")]
    #[test_case(1, "...\n.#.\n..1"; "part with bottom-right number returns number")]
    #[test_case(3, "1.2\n.#.\n..."; "part with multiple numbers returns sum")]
    #[test_case(3, "1.2\n#.#\n..."; "multiple parts with multiple numbers returns sum")]
    #[test_case(0, "1..\n...\n..."; "no parts for number returns zero")]
    #[test_case(12345, "12345\n..#..\n....."; "part with number exceeding both sides returns number")]
    fn part_numbers_sum(expected: u32, input: &str) {
        assert_eq!(
            expected,
            parts(input)
                .into_iter()
                .flat_map(|part| part.numbers)
                .sum::<u32>()
        );
    }

    #[test]
    fn gear_ratios_sum() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(
            467835,
            parts(input)
                .into_iter()
                .filter_map(|part| part.try_into().ok())
                .map(|gear: Gear| gear.ratio())
                .sum::<u64>()
        );
    }
}
