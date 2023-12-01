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
    use test_case::test_case;

    #[test_case(77, "7"; "single digit")]
    #[test_case(77, "7a"; "single digit at the start")]
    #[test_case(77, "a7"; "single digit at the end")]
    #[test_case(78, "7a8b"; "first digit at the start")]
    #[test_case(78, "a7b8"; "last digit at the end")]
    #[test_case(78, "a78b"; "both digits surrounded")]
    #[test_case(79, "7a8b9c"; "more than two digits")]

    fn calibration_value_tests(expected: u32, input: &str) {
        assert_eq!(expected, calibration_value(input));
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
