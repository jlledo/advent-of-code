use aho_corasick::AhoCorasick;
use color_eyre::eyre::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .expect("first argument should be the input file");
    let input = std::fs::read_to_string(input_file)?;

    let part_1_total: u32 = input.lines().map(calibration_value_part_1).sum();
    println!("Part 1: {part_1_total}");

    let part_2_total: u32 = input.lines().map(calibration_value_part_2).sum();
    println!("Part 2 (Regex): {part_2_total}");

    let part_2_ac_total: u32 = input.lines().map(calibration_value_part_2_ac).sum();
    println!("Part 2 (Aho-Corasick): {part_2_ac_total}");

    Ok(())
}

fn calibration_value_part_1(string: &str) -> u32 {
    let tens = find_number(string.chars()).unwrap();
    let units = find_number(string.chars().rev()).unwrap();

    (tens * 10) + units
}

fn find_number(mut chars: impl Iterator<Item = char>) -> Option<u32> {
    chars
        .find(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
}

fn calibration_value_part_2(string: &str) -> u32 {
    let first_regex =
        regex::Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d).*").unwrap();
    let tens = first_regex
        .captures_iter(string)
        .map(|c| c.extract::<1>().1[0])
        .next()
        .unwrap();
    let tens = parse(tens).unwrap();

    let last_regex = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let units = last_regex
        .captures_iter(string)
        .map(|c| c.extract::<1>().1[0])
        .next()
        .unwrap();
    let units = parse(units).unwrap();

    (tens * 10) + units
}

fn parse(string: &str) -> Option<u32> {
    match string {
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}

fn calibration_value_part_2_ac(string: &str) -> u32 {
    let number_strings = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let number_values: [u32; 18] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let ac = AhoCorasick::new(number_strings).unwrap();
    let matches: Vec<u32> = ac
        .find_overlapping_iter(string)
        .map(|m| number_values[m.pattern().as_usize()])
        .collect();

    (matches.first().unwrap() * 10) + matches.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(38, "pqr3stu8vwx"; "two digits surrounded")]
    #[test_case(15, "a1b2c3d4e5f"; "more than two digits")]
    #[test_case(77, "treb7uchet"; "single surrounded digit")]
    #[test_case(77, "7"; "single naked character")]

    fn calibration_value_part_1_tests(expected: u32, input: &str) {
        assert_eq!(expected, calibration_value_part_1(input));
    }

    #[test_case(38, "pqr3stu8vwx"; "two digits surrounded")]
    #[test_case(15, "a1b2c3d4e5f"; "more than two digits")]
    #[test_case(77, "treb7uchet"; "single surrounded digit")]
    #[test_case(77, "7"; "single naked character")]
    #[test_case(29, "two1nine"; "two words")]
    #[test_case(83, "eightwothree"; "more than two words")]
    #[test_case(13, "abcone2threexyz"; "surrounded words")]
    #[test_case(24, "xtwone3four"; "overlapping first word")]
    #[test_case(42, "4nineeightseven2"; "two external digits with words")]
    #[test_case(14, "zoneight234"; "word and digit")]
    #[test_case(76, "7pqrstsixteen"; "invalid word")]
    fn calibration_value_part_2_tests(expected: u32, input: &str) {
        assert_eq!(expected, calibration_value_part_2(input));
    }

    #[test_case(38, "pqr3stu8vwx"; "two digits surrounded")]
    #[test_case(15, "a1b2c3d4e5f"; "more than two digits")]
    #[test_case(77, "treb7uchet"; "single surrounded digit")]
    #[test_case(77, "7"; "single naked character")]
    #[test_case(29, "two1nine"; "two words")]
    #[test_case(83, "eightwothree"; "more than two words")]
    #[test_case(13, "abcone2threexyz"; "surrounded words")]
    #[test_case(24, "xtwone3four"; "overlapping first word")]
    #[test_case(42, "4nineeightseven2"; "two external digits with words")]
    #[test_case(14, "zoneight234"; "word and digit")]
    #[test_case(76, "7pqrstsixteen"; "invalid word")]
    fn calibration_value_part_2_ac_tests(expected: u32, input: &str) {
        assert_eq!(expected, calibration_value_part_2_ac(input));
    }
}
