use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let grid = self.parse_grid();
        let grid = tilt_grid(&grid);
        total_load(&grid).to_string()
    }

    fn solve_part_2(&self) -> String {
        let target = 1_000_000_000;
        let mut steps = 0;
        let mut grid = self.parse_grid();
        let mut seen = HashMap::new();
        while steps < target {
            for _ in 0..4 {
                grid = tilt_grid(&grid);
                grid = rotate_grid(&grid);
            }
            steps += 1;
            match seen.get(&grid) {
                Some(&prev_steps) => {
                    let cycle_length = steps - prev_steps;
                    let cycles_to_skip = (target - steps) / cycle_length;
                    steps += cycles_to_skip * cycle_length;
                }
                None => {
                    seen.insert(grid.clone(), steps);
                }
            }
        }
        total_load(&grid).to_string()
    }
}

fn rotate_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![vec!['.'; grid.len()]; grid[0].len()];
    for (row, line) in grid.iter().enumerate() {
        for (col, &c) in line.iter().enumerate() {
            new_grid[col][grid.len() - row - 1] = c;
        }
    }
    new_grid
}

fn tilt_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut highest_occupied = vec![-1; grid[0].len()];
    let mut new_grid = grid.clone();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            match new_grid[row][col] {
                'O' => {
                    highest_occupied[col] += 1;
                    new_grid[row][col] = '.';
                    new_grid[highest_occupied[col] as usize][col] = 'O';
                }
                '#' => highest_occupied[col] = row as i32,
                _ => {}
            }
        }
    }
    new_grid.clone()
}

fn total_load(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (grid.len() - i))
        .sum()
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_grid(&self) -> Vec<Vec<char>> {
        self.input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "O....#....\n\
            O.OO#....#\n\
            .....##...\n\
            OO.#O....O\n\
            .O.....O#.\n\
            O.#..O.#.#\n\
            ..O..#O..O\n\
            .......O..\n\
            #....###..\n\
            #OO..#....";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "136");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/14")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "113486");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "O....#....\n\
            O.OO#....#\n\
            .....##...\n\
            OO.#O....O\n\
            .O.....O#.\n\
            O.#..O.#.#\n\
            ..O..#O..O\n\
            .......O..\n\
            #....###..\n\
            #OO..#....";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "64");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/14")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "104409");
    }
}
