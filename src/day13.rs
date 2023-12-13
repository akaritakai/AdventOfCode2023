use crate::puzzle::Puzzle;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        self.parse_mirrors()
            .iter()
            .map(|(rows, cols)| {
                if let Some(i) = find_pure_reflection(rows) {
                    100 * (i + 1)
                } else if let Some(i) = find_pure_reflection(cols) {
                    i + 1
                } else {
                    unreachable!()
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.parse_mirrors()
            .iter()
            .map(|(rows, cols)| {
                if let Some(i) = find_smudged_reflection(rows) {
                    100 * (i + 1)
                } else if let Some(i) = find_smudged_reflection(cols) {
                    i + 1
                } else {
                    unreachable!()
                }
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

    fn parse_mirrors(&self) -> Vec<(Vec<u64>, Vec<u64>)> {
        self.input
            .split("\n\n")
            .map(|block| {
                let lines: Vec<&str> = block.lines().collect();
                let rows = lines
                    .iter()
                    .map(|line| {
                        line.chars().fold(0u64, |mut row, c| {
                            row <<= 1;
                            if c == '#' {
                                row |= 1;
                            }
                            row
                        })
                    })
                    .collect::<Vec<u64>>();
                let mut cols = vec![0u64; lines[0].len()];
                for (_, &row) in rows.iter().enumerate() {
                    for (j, col) in cols.iter_mut().enumerate() {
                        *col <<= 1;
                        *col |= (row >> (lines[0].len() - j - 1)) & 1;
                    }
                }
                (rows, cols)
            })
            .collect()
    }
}

fn find_pure_reflection(values: &[u64]) -> Option<usize> {
    'OUTER: for (i, pair) in values.iter().zip(values.iter().skip(1)).enumerate() {
        if pair.0 != pair.1 {
            continue;
        }
        let mut k = 1;
        while i as i32 - k >= 0 && i as i32 + k + 1 < values.len() as i32 {
            if values[i - k as usize] != values[i + k as usize + 1] {
                continue 'OUTER;
            }
            k += 1;
        }
        return Some(i);
    }
    None
}

fn find_smudged_reflection(values: &[u64]) -> Option<usize> {
    'OUTER: for (i, pair) in values.iter().zip(values.iter().skip(1)).enumerate() {
        let mut used_smudge = false;
        if pair.0 != pair.1 && (pair.0 ^ pair.1).count_ones() != 1 {
            continue;
        } else if pair.0 != pair.1 {
            used_smudge = true;
        }
        let mut k = 1;
        while i as i32 - k >= 0 && i as i32 + k + 1 < values.len() as i32 {
            let left = values[i - k as usize];
            let right = values[i + k as usize + 1];
            if left != right && (left ^ right).count_ones() != 1 {
                continue 'OUTER;
            } else if left != right {
                if used_smudge {
                    continue 'OUTER;
                }
                used_smudge = true;
            }
            k += 1;
        }
        if used_smudge {
            return Some(i);
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
