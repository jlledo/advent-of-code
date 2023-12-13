fn main() -> color_eyre::eyre::Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .expect("first argument should be the input file");
    let input = std::fs::read_to_string(input_file)?;

    let part_numbers_sum = all_part_numbers(&input).into_iter().sum::<u32>();
    println!("Sum of part numbers: {part_numbers_sum}");

    Ok(())
}

fn all_part_numbers<'a>(input: &'a str) -> Vec<u32> {
    let mut all_part_numbers = vec![];
    let lines: Vec<&'a str> = input.lines().collect();
    for (row, line) in lines.iter().enumerate() {
        for (column, c) in line.as_bytes().iter().enumerate() {
            if is_part(*c) {
                all_part_numbers.append(&mut part_numbers(&lines, row, column));
            }
        }
    }

    all_part_numbers
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

#[cfg(test)]
mod tests {
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
            super::all_part_numbers(input).into_iter().sum::<u32>()
        );
    }
}
