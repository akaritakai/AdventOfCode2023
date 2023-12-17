use crate::puzzle::Puzzle;
use rayon::prelude::*;
use std::collections::HashSet;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let grid = self.parse_grid();
        count_tiles_energized(&grid, (0, 0, 0, 1)).to_string()
    }

    fn solve_part_2(&self) -> String {
        let grid = self.parse_grid();
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut beams = Vec::new();
        beams.extend((0..cols).map(|col| (0, col, 1, 0)).collect::<Vec<_>>());
        beams.extend(
            (0..cols)
                .map(|col| (rows - 1, col, -1, 0))
                .collect::<Vec<_>>(),
        );
        beams.extend((0..rows).map(|row| (row, 0, 0, 1)).collect::<Vec<_>>());
        beams.extend(
            (0..rows)
                .map(|row| (row, cols - 1, 0, -1))
                .collect::<Vec<_>>(),
        );
        beams
            .par_iter()
            .map(|&start| count_tiles_energized(&grid, start))
            .max()
            .unwrap()
            .to_string()
    }
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

fn count_tiles_energized(grid: &[Vec<char>], start: (i32, i32, i32, i32)) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut beams = vec![start];
    let mut energized = HashSet::new();
    while let Some((row, col, drow, dcol)) = beams.pop() {
        if row < 0 || row >= rows as i32 || col < 0 || col >= cols as i32 {
            continue;
        }
        if !energized.insert((row, col, drow, dcol)) {
            continue;
        }
        // Check what the beam is hitting.
        match grid[row as usize][col as usize] {
            '.' => {
                beams.push((row + drow, col + dcol, drow, dcol));
            }
            '/' => {
                beams.push((row - dcol, col - drow, -dcol, -drow));
            }
            '\\' => {
                beams.push((row + dcol, col + drow, dcol, drow));
            }
            '|' => {
                if dcol == 0 {
                    beams.push((row + drow, col + dcol, drow, dcol));
                } else {
                    beams.push((row + 1, col, 1, 0));
                    beams.push((row - 1, col, -1, 0));
                }
            }
            '-' => {
                if drow == 0 {
                    beams.push((row + drow, col + dcol, drow, dcol));
                } else {
                    beams.push((row, col + 1, 0, 1));
                    beams.push((row, col - 1, 0, -1));
                }
            }
            _ => (),
        }
    }
    energized
        .iter()
        .map(|(row, col, _, _)| (row, col))
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = ".|...\\....\n\
            |.-.\\.....\n\
            .....|-...\n\
            ........|.\n\
            ..........\n\
            .........\\\n\
            ..../.\\\\..\n\
            .-.-/..|..\n\
            .|....-|.\\\n\
            ..//.|....";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "46");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/16")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "7482");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = ".|...\\....\n\
            |.-.\\.....\n\
            .....|-...\n\
            ........|.\n\
            ..........\n\
            .........\\\n\
            ..../.\\\\..\n\
            .-.-/..|..\n\
            .|....-|.\\\n\
            ..//.|....";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "51");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/16")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "7896");
    }
}
