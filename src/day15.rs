use crate::puzzle::Puzzle;
use lazy_regex::regex_captures;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        self.parse_steps()
            .iter()
            .map(|s| hash_algorithm(s))
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
        for s in self.parse_steps() {
            let (_, label, operation, digit) = regex_captures!(r"(\S+)(=|-)(\d+)?", &s).unwrap();
            let hash = hash_algorithm(label);
            match operation {
                "-" => boxes[hash].retain(|(l, _)| l != label),
                "=" => {
                    let digit = digit.parse::<usize>().unwrap();
                    match boxes[hash].iter_mut().find(|(l, _)| l == label) {
                        Some((_, d)) => *d = digit,
                        None => boxes[hash].push((label.to_string(), digit)),
                    }
                }
                _ => unreachable!(),
            };
        }
        boxes
            .iter()
            .enumerate()
            .flat_map(|(i, b)| {
                b.iter()
                    .enumerate()
                    .map(move |(j, (_, d))| (i + 1) * (j + 1) * d)
            })
            .sum::<usize>()
            .to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_steps(&self) -> Vec<String> {
        self.input.split(',').map(|s| s.replace('\n', "")).collect()
    }
}

fn hash_algorithm(s: &str) -> usize {
    s.bytes().fold(0, |hash, c| (hash + c as usize) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1320");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/15")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "511215");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "145");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/15")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "236057");
    }
}
