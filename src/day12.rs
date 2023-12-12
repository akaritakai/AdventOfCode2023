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
    let mut memo = HashMap::new();
    let line = format!("{}.", line).to_string();
    count_arrangements_impl(&mut memo, line.as_bytes(), counts, (0, 0, 0))
}

fn count_arrangements_impl(
    memo: &mut HashMap<(usize, usize, usize), usize>,
    line: &[u8],
    counts: &[usize],
    key: (usize, usize, usize),
) -> usize {
    if let Some(&result) = memo.get(&key) {
        return result;
    }
    let result;
    let (pos, group, count) = key;
    // Base case: we've reached the end of the line
    if pos == line.len() {
        // If we've seen all the groups after reading the entire string, we've found a valid arrangement. Otherwise, we haven't.
        if counts.len() == group && count == 0 {
            result = 1;
        } else {
            result = 0;
        }
    }
    // Base invalid cases: we've seen too many groups, or we've seen too many '#'s for the current group
    else if group > counts.len()
        || (group == counts.len() && count != 0)
        || (group < counts.len() && count > counts[group])
    {
        result = 0;
    }
    // Case 1: Current character is '#' -- add it to our current group's count
    else if line[pos] == b'#' {
        result = count_arrangements_impl(memo, line, counts, (pos + 1, group, count + 1));
    }
    // Case 2: Current character is '.'
    else if line[pos] == b'.' {
        // Case 2a: Check if the group is valid
        if group < counts.len() && count == counts[group] {
            result = count_arrangements_impl(memo, line, counts, (pos + 1, group + 1, 0));
        }
        // Case 2b: We haven't seen any '#'s yet for the current group
        else if count == 0 {
            result = count_arrangements_impl(memo, line, counts, (pos + 1, group, 0));
        }
        // Case 2c: We've seen some '#'s for the current group, but not enough
        else {
            result = 0;
        }
    }
    // Case 3: Wildcard '?'
    else {
        let mut hash_count = 0;
        let mut dot_count = 0;

        // Assume we have a '#' here and it can be added ot the current group
        if group < counts.len() && count < counts[group] {
            hash_count = count_arrangements_impl(memo, line, counts, (pos + 1, group, count + 1));
        }
        if group < counts.len() && count == counts[group] {
            // Assume we have a '.' here and we're ready to move into the next group
            dot_count = count_arrangements_impl(memo, line, counts, (pos + 1, group + 1, 0));
        } else if count == 0 {
            // Assume we have a '.' here and we're staying in the current group
            dot_count = count_arrangements_impl(memo, line, counts, (pos + 1, group, 0));
        }

        result = hash_count + dot_count;
    }
    memo.insert(key, result);
    result
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
