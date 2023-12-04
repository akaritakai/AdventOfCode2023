use crate::puzzle::Puzzle;
use lazy_regex::regex;
use std::collections::HashSet;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn day(&self) -> u8 {
        4
    }

    fn solve_part_1(&self) -> String {
        self.input
            .lines()
            .map(parse_card)
            .map(|n| match n {
                0 => 0,
                _ => 1 << (n - 1),
            })
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let numbers = self.input.lines().map(parse_card).collect::<Vec<usize>>();
        let mut counts = vec![1; numbers.len()];
        for i in 0..counts.len() {
            for j in i + 1..=i + numbers[i] {
                counts[j] += counts[i];
            }
        }
        counts.iter().sum::<usize>().to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }
}

fn parse_card(line: &str) -> usize {
    let re = regex!(r".*:(.*)\|(.*)");
    let cap = re.captures(line).unwrap();

    let mut winning_numbers = HashSet::new();
    for m in regex!(r"\d+").find_iter(&cap[1]) {
        winning_numbers.insert(m.as_str().parse::<usize>().unwrap());
    }

    let mut num_winning_numbers = 0;
    for m in regex!(r"\d+").find_iter(&cap[2]) {
        let number = m.as_str().parse::<usize>().unwrap();
        if winning_numbers.contains(&number) {
            num_winning_numbers += 1;
        }
    }
    num_winning_numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "13");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/04")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "33950");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "30");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/04")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "14814534");
    }
}
