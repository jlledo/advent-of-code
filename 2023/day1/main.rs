use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .expect("first argument should be the input file");
    let input = std::fs::read_to_string(input_file)?;

    let total: u32 = calibration_value_sum(&input);
    println!("Part 1: {}", total);

    Ok(())
}

fn calibration_value_sum(string: &str) -> u32 {
    string.lines().map(calibration_value).sum()
}

fn calibration_value(string: &str) -> u32 {
    let tens = find_number(string.chars()).unwrap();
    let units = find_number(string.chars().rev()).unwrap();

    (tens * 10) + units
}

fn find_number(mut chars: impl Iterator<Item = char>) -> Option<u32> {
    chars
        .find(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_digit() {
        assert_eq!(77, calibration_value_sum("7"));
    }

    #[test]
    fn single_digit_at_start() {
        assert_eq!(77, calibration_value_sum("7a"));
    }

    #[test]
    fn single_digit_at_end() {
        assert_eq!(77, calibration_value_sum("a7"));
    }

    #[test]
    fn more_than_two_digits() {
        assert_eq!(59, calibration_value("bib5asj2dfb9"))
    }

    #[test]
    fn first_digit_at_start() {
        assert_eq!(78, calibration_value_sum("7a8b"));
    }

    #[test]
    fn last_digit_at_end() {
        assert_eq!(78, calibration_value_sum("a7b8"));
    }

    #[test]
    fn both_digits_surrounded() {
        assert_eq!(78, calibration_value("a78b"));
    }

    #[test]
    fn multiple_lines() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, calibration_value_sum(input));
    }
}
