use crate::puzzle::Puzzle;
use num::integer::binomial;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        self.input
            .lines()
            .map(parse_line)
            .map(successor)
            .sum::<i64>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.input
            .lines()
            .map(parse_line)
            .map(predecessor)
            .sum::<i64>()
            .to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn successor(nums: Vec<i64>) -> i64 {
    nums.iter()
        .enumerate()
        .map(|(i, &x)| {
            let sign = if (nums.len() - 1 - i) % 2 == 0 { 1 } else { -1 };
            sign * x * binomial(nums.len(), i) as i64
        })
        .sum()
}

fn predecessor(nums: Vec<i64>) -> i64 {
    nums.iter()
        .enumerate()
        .map(|(i, &x)| {
            let sign = if i % 2 == 0 { 1 } else { -1 };
            sign * x * binomial(nums.len(), i + 1) as i64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "0 3 6 9 12 15\n\
            1 3 6 10 15 21\n\
            10 13 16 21 30 45";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "114");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/09")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1684566095");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "0 3 6 9 12 15\n\
            1 3 6 10 15 21\n\
            10 13 16 21 30 45";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "2");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/09")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1136");
    }
}
