use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .expect("first argument should be the input file");
    let input = std::fs::read_to_string(input_file)?;

    let total: u32 = input.lines().map(calibration_value).sum();
    println!("Part 1: {}", total);

    Ok(())
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

    #[test_case(38, "pqr3stu8vwx"; "two digits surrounded")]
    #[test_case(15, "a1b2c3d4e5f"; "more than two digits")]
    #[test_case(77, "treb7uchet"; "single surrounded digit")]
    #[test_case(77, "7"; "single naked character")]

    fn calibration_value_tests(expected: u32, input: &str) {
        assert_eq!(expected, calibration_value(input));
    }
}
