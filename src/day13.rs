use crate::puzzle::Puzzle;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        self.solve_generic(0).to_string()
    }

    fn solve_part_2(&self) -> String {
        self.solve_generic(1).to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_mirrors(&self) -> Vec<(Vec<u64>, Vec<u64>)> {
        self.input.split("\n\n").map(parse_mirror).collect()
    }

    fn solve_generic(&self, expected_diffs: u32) -> usize {
        self.parse_mirrors()
            .iter()
            .map(|(rows, cols)| {
                reflection_score(rows, expected_diffs, 100)
                    .or_else(|| reflection_score(cols, expected_diffs, 1))
                    .unwrap()
            })
            .sum::<usize>()
    }
}

fn parse_mirror(block: &str) -> (Vec<u64>, Vec<u64>) {
    let grid: Vec<Vec<u64>> = block
        .lines()
        .map(|line| line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();
    let rows = grid
        .iter()
        .map(|row| row.iter().fold(0, |acc, &bit| (acc << 1) | bit))
        .collect::<Vec<u64>>();
    let cols = (0..grid[0].len())
        .map(|i| grid.iter().fold(0, |acc, row| (acc << 1) | row[i]))
        .collect::<Vec<u64>>();
    (rows, cols)
}

fn reflection_score(values: &[u64], expected_diffs: u32, factor: usize) -> Option<usize> {
    for i in 0..values.len() - 1 {
        let mut diffs = 0;
        for (j, k) in (0..=i).rev().zip((i + 1)..values.len()) {
            diffs += (values[j] ^ values[k]).count_ones();
        }
        if diffs == expected_diffs {
            return Some((i + 1) * factor);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "#.##..##.\n\
            ..#.##.#.\n\
            ##......#\n\
            ##......#\n\
            ..#.##.#.\n\
            ..##..##.\n\
            #.#.##.#.\n\
            \n\
            #...##..#\n\
            #....#..#\n\
            ..##..###\n\
            #####.##.\n\
            #####.##.\n\
            ..##..###\n\
            #....#..#";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "405");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/13")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "27202");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "#.##..##.\n\
            ..#.##.#.\n\
            ##......#\n\
            ##......#\n\
            ..#.##.#.\n\
            ..##..##.\n\
            #.#.##.#.\n\
            \n\
            #...##..#\n\
            #....#..#\n\
            ..##..###\n\
            #####.##.\n\
            #####.##.\n\
            ..##..###\n\
            #....#..#";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "400");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/13")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "41566");
    }
}
