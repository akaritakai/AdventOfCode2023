use crate::puzzle::Puzzle;
use lazy_regex::regex;
use std::collections::HashMap;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn day(&self) -> u8 {
        3
    }

    fn solve_part_1(&self) -> String {
        self.get_edges()
            .values()
            .flatten()
            .map(|number| number.num)
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.get_edges()
            .iter()
            .filter_map(|(symbol, numbers)| {
                if symbol.c == '*' && numbers.len() == 2 {
                    Some(numbers)
                } else {
                    None
                }
            })
            .map(|numbers| numbers.iter().map(|number| number.num).product::<usize>())
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

    fn get_edges(&self) -> HashMap<Symbol, Vec<PartNumber>> {
        // Discover all part numbers.
        let mut numbers = Vec::new();
        for (row, line) in self.input.lines().enumerate() {
            for cap in regex!(r"\d+").find_iter(line) {
                let num = cap.as_str().parse::<usize>().unwrap();
                numbers.push(PartNumber {
                    num,
                    row,
                    start_col: cap.start(),
                    end_col: cap.end(),
                });
            }
        }

        // Discover all symbols.
        let mut symbols = Vec::new();
        for (row, line) in self.input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c != '.' && !c.is_ascii_digit() {
                    symbols.push(Symbol { c, row, col });
                }
            }
        }

        // Map which numbers touch which symbols.
        let mut map = HashMap::new();
        for symbol in symbols {
            let mut adjacent_numbers = Vec::new();
            for number in &numbers {
                if is_adjacent(&symbol, number) {
                    adjacent_numbers.push(*number);
                }
            }
            map.insert(symbol, adjacent_numbers);
        }
        map
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct PartNumber {
    num: usize,
    row: usize,
    start_col: usize,
    end_col: usize,
}

#[derive(Eq, PartialEq, Hash)]
struct Symbol {
    c: char,
    row: usize,
    col: usize,
}

fn is_adjacent(symbol: &Symbol, number: &PartNumber) -> bool {
    for i in number.start_col..number.end_col {
        if (symbol.row as i32 - number.row as i32).abs() <= 1
            && (symbol.col as i32 - i as i32).abs() <= 1
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
           .664.598..";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "4361");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/03")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "556367");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
           .664.598..";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "467835");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/03")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "89471771");
    }
}
