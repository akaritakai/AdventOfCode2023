use crate::day14::Direction::{East, North, South, West};
use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let mut grid = self.parse_grid();
        tilt_grid(&mut grid, North);
        total_load(&grid).to_string()
    }

    fn solve_part_2(&self) -> String {
        let target = 1_000_000_000;
        let mut steps = 0;
        let mut grid = self.parse_grid();
        let mut seen = HashMap::new();
        while steps < target {
            tilt_grid(&mut grid, North);
            tilt_grid(&mut grid, West);
            tilt_grid(&mut grid, South);
            tilt_grid(&mut grid, East);
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

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

struct TiltConfig {
    occupied_init: i32,
    occupied_step: i32,
    major_start: i32,
    major_end: i32,
    major_step: i32,
    minor: usize,
    row_major: bool,
}

fn tilt_grid(grid: &mut [Vec<char>], dir: Direction) {
    let (rows, cols) = (grid.len(), grid[0].len());
    match dir {
        North => (0..cols).for_each(|col| {
            tilt_grid_impl(
                grid,
                TiltConfig {
                    occupied_init: -1,
                    occupied_step: 1,
                    major_start: 0,
                    major_end: rows as i32,
                    major_step: 1,
                    minor: col,
                    row_major: true,
                },
            )
        }),
        South => (0..cols).for_each(|col| {
            tilt_grid_impl(
                grid,
                TiltConfig {
                    occupied_init: cols as i32,
                    occupied_step: -1,
                    major_start: (rows - 1) as i32,
                    major_end: -1,
                    major_step: -1,
                    minor: col,
                    row_major: true,
                },
            )
        }),
        West => (0..rows).for_each(|row| {
            tilt_grid_impl(
                grid,
                TiltConfig {
                    occupied_init: -1,
                    occupied_step: 1,
                    major_start: 0,
                    major_end: cols as i32,
                    major_step: 1,
                    minor: row,
                    row_major: false,
                },
            )
        }),
        East => (0..rows).for_each(|row| {
            tilt_grid_impl(
                grid,
                TiltConfig {
                    occupied_init: rows as i32,
                    occupied_step: -1,
                    major_start: (cols - 1) as i32,
                    major_end: -1,
                    major_step: -1,
                    minor: row,
                    row_major: false,
                },
            )
        }),
    };
}

fn tilt_grid_impl(grid: &mut [Vec<char>], config: TiltConfig) {
    let mut occupied = config.occupied_init;
    let mut major = config.major_start;
    while major != config.major_end {
        let (row, col) = if config.row_major {
            (major as usize, config.minor)
        } else {
            (config.minor, major as usize)
        };
        match grid[row][col] {
            'O' => {
                occupied += config.occupied_step;
                grid[row][col] = '.';
                if config.row_major {
                    grid[occupied as usize][config.minor] = 'O';
                } else {
                    grid[config.minor][occupied as usize] = 'O';
                };
            }
            '#' => occupied = major,
            _ => {}
        }
        major += config.major_step;
    }
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
