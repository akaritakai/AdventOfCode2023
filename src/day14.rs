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
            steps += 1;
            for _ in 0..4 {
                grid = tilt_grid(&grid);
                grid = rotate_grid(&grid);
            }

            if let Some(&prev_steps) = seen.get(&grid) {
                let cycle_length = steps - prev_steps;
                let amount = (target - steps) / cycle_length;
                steps += amount * cycle_length;
            }
            seen.insert(grid.clone(), steps);
        }
        total_load(&grid).to_string()
    }
}

fn rotate_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut new_grid = vec![vec!['.'; num_rows]; num_cols];
    for (row, line) in grid.iter().enumerate() {
        for (col, &c) in line.iter().enumerate() {
            new_grid[col][num_rows - row - 1] = c;
        }
    }
    new_grid
}

fn tilt_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut highest_occupied = vec![-1; grid[0].len()];
    let mut new_grid = grid.clone();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if new_grid[row][col] == 'O' {
                highest_occupied[col] += 1;
                new_grid[row][col] = '.';
                new_grid[highest_occupied[col] as usize][col] = 'O';
            } else if new_grid[row][col] == '#' {
                highest_occupied[col] = row as i32;
            }
        }
    }
    new_grid.clone()
}

fn total_load(grid: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    for (i, row) in grid.iter().enumerate() {
        let factor = grid.len() - i;
        for &c in row {
            if c == 'O' {
                load += factor;
            }
        }
    }
    load
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
