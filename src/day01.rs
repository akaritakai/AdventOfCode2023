use crate::puzzle::Puzzle;
use lazy_regex::regex;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn day(&self) -> u8 {
        1
    }

    fn solve_part_1(&self) -> String {
        self.solve_internal(false).to_string()
    }

    fn solve_part_2(&self) -> String {
        self.solve_internal(true).to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn solve_internal(&self, allow_spelled_out: bool) -> i32 {
        self.input
            .lines()
            .map(|line| extract_calibration_value(extract_digits(line, allow_spelled_out)))
            .sum()
    }
}

fn extract_calibration_value(digits: Vec<i32>) -> i32 {
    let first_digit = digits.first().unwrap();
    let last_digit = digits.last().unwrap();
    (10 * first_digit) + last_digit
}

fn extract_digits(line: &str, allow_spelled_out: bool) -> Vec<i32> {
    let re = if allow_spelled_out {
        regex!(r"^(one|two|three|four|five|six|seven|eight|nine|\d)")
    } else {
        regex!(r"^(\d)")
    };
    let mut digits = Vec::new();
    for i in 0..line.len() {
        if let Some(m) = re.find(&line[i..]) {
            let digit = match m.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => m.as_str().parse::<i32>().unwrap(),
            };
            digits.push(digit);
        }
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "142");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/01")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "55002");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "281");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/01")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "55093");
    }
}
