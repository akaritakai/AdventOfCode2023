use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        self.parse_input()
            .iter()
            .map(|(pattern, counts)| count_arrangements(pattern, counts))
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.parse_input()
            .iter()
            .map(|(pattern, counts)| {
                let pattern = [*pattern; 5].join("?");
                let counts = counts.repeat(5);
                count_arrangements(&pattern, &counts)
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

    fn parse_input(&self) -> Vec<(&str, Vec<usize>)> {
        self.input
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let pattern = parts.next().unwrap();
                let counts = parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                (pattern, counts)
            })
            .collect()
    }
}

fn count_arrangements(line: &str, counts: &[usize]) -> usize {
    let line = line.as_bytes();
    let n = line.len();
    let m = counts.len();
    let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];

    dp[n][m][0] = 1;
    dp[n][m - 1][counts[m - 1]] = 1;

    for pos in (0..n).rev() {
        for group in 0..=m {
            let max_count = if group < m { counts[group] } else { 0 };
            for count in 0..=max_count {
                for &c in &[b'.', b'#'] {
                    if line[pos] == c || line[pos] == b'?' {
                        if c == b'.' && count == 0 {
                            dp[pos][group][count] += dp[pos + 1][group][0];
                        } else if c == b'.' && count > 0 && group < m && counts[group] == count {
                            dp[pos][group][count] += dp[pos + 1][group + 1][0];
                        } else if c == b'#' {
                            dp[pos][group][count] += dp[pos + 1][group][count + 1];
                        }
                    }
                }
            }
        }
    }

    dp[0][0][0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "???.### 1,1,3\n\
            .??..??...?##. 1,1,3\n\
            ?#?#?#?#?#?#?#? 1,3,1,6\n\
            ????.#...#... 4,1,1\n\
            ????.######..#####. 1,6,5\n\
            ?###???????? 3,2,1";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "21");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/12")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "7541");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "???.### 1,1,3\n\
            .??..??...?##. 1,1,3\n\
            ?#?#?#?#?#?#?#? 1,3,1,6\n\
            ????.#...#... 4,1,1\n\
            ????.######..#####. 1,6,5\n\
            ?###???????? 3,2,1";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "525152");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/12")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "17485169859432");
    }
}
