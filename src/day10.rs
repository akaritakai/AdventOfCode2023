use crate::puzzle::Puzzle;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let pipes = Pipes::parse(&self.input);
        let boundary_point = pipes.find_starting_point();
        let mut direction = pipes.find_initial_direction(boundary_point);
        let mut position = boundary_point + direction;
        let mut steps = 1;
        while pipes[position] != 'S' {
            direction = next_direction(pipes[position], direction);
            position += direction;
            steps += 1;
        }
        (steps / 2).to_string()
    }

    fn solve_part_2(&self) -> String {
        let pipes = Pipes::parse(&self.input);
        let mut boundary_point = pipes.find_starting_point();
        let mut direction = pipes.find_initial_direction(boundary_point);
        let mut position = boundary_point + direction;
        let mut steps = 1;
        let mut area = 0;
        while pipes[position] != 'S' {
            if matches!(pipes[position], 'L' | 'J' | '7' | 'F') {
                area += boundary_point.row * position.col - boundary_point.col * position.row;
                boundary_point = position;
            }
            direction = next_direction(pipes[position], direction);
            position += direction;
            steps += 1;
        }
        area += boundary_point.row * position.col - boundary_point.col * position.row;
        (area.abs() / 2 - steps / 2 + 1).to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }
}

struct Pipes {
    grid: Vec<Vec<char>>,
}

impl Pipes {
    fn parse(input: &str) -> Pipes {
        Pipes {
            grid: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn num_rows(&self) -> i32 {
        self.grid.len() as i32
    }

    fn num_cols(&self) -> i32 {
        self.grid[0].len() as i32
    }

    fn find_starting_point(&self) -> Point {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(row, line)| {
                line.iter().enumerate().find_map(|(col, &ch)| {
                    if ch == 'S' {
                        Some(Point {
                            row: row as i32,
                            col: col as i32,
                        })
                    } else {
                        None
                    }
                })
            })
            .unwrap()
    }

    fn find_initial_direction(&self, start: Point) -> Point {
        if start.row > 0 && matches!(self[start + NORTH], '|' | '7' | 'F') {
            NORTH
        } else if start.row < self.num_rows() - 1 && matches!(self[start + SOUTH], '|' | 'L' | 'J')
        {
            SOUTH
        } else if start.col < self.num_cols() - 1 && matches!(self[start + EAST], '-' | 'L' | 'F') {
            EAST
        } else if start.col > 0 && matches!(self[start + WEST], '-' | 'J' | '7') {
            WEST
        } else {
            unreachable!();
        }
    }
}

impl std::ops::Index<Point> for Pipes {
    type Output = char;
    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.row as usize][index.col as usize]
    }
}

const NORTH: Point = Point { row: -1, col: 0 };
const SOUTH: Point = Point { row: 1, col: 0 };
const EAST: Point = Point { row: 0, col: 1 };
const WEST: Point = Point { row: 0, col: -1 };

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    row: i32,
    col: i32,
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = *self + other;
    }
}

fn next_direction(tile: char, direction: Point) -> Point {
    match tile {
        'L' if direction == SOUTH => EAST,
        'L' if direction == WEST => NORTH,
        'J' if direction == SOUTH => WEST,
        'J' if direction == EAST => NORTH,
        '7' if direction == NORTH => WEST,
        '7' if direction == EAST => SOUTH,
        'F' if direction == NORTH => EAST,
        'F' if direction == WEST => SOUTH,
        _ => direction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = ".....\n\
            .S-7.\n\
            .|.|.\n\
            .L-J.\n\
            .....";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "4");
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "..F7.\n\
            .FJ|.\n\
            SJ.L7\n\
            |F--J\n\
            LJ...";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "8");
    }

    #[test]
    fn test_part_1_example_3() {
        let input = "7-F7-\n\
            .FJ|7\n\
            SJLL7\n\
            |F--J\n\
            LJ.LJ";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "8");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/10")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "6733");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "...........\n\
            .S-------7.\n\
            .|F-----7|.\n\
            .||.....||.\n\
            .||.....||.\n\
            .|L-7.F-J|.\n\
            .|..|.|..|.\n\
            .L--J.L--J.\n\
            ...........";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "4");
    }

    #[test]
    fn test_part_2_example_2() {
        let input = "..........\n\
            .S------7.\n\
            .|F----7|.\n\
            .||....||.\n\
            .||....||.\n\
            .|L-7F-J|.\n\
            .|..||..|.\n\
            .L--JL--J.\n\
            ..........";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "4");
    }

    #[test]
    fn test_part_2_example_3() {
        let input = ".F----7F7F7F7F-7....\n\
            .|F--7||||||||FJ....\n\
            .||.FJ||||||||L7....\n\
            FJL7L7LJLJ||LJ.L-7..\n\
            L--J.L7...LJS7F-7L7.\n\
            ....F-J..F7FJ|L7L7L7\n\
            ....L7.F7||L7|.L7L7|\n\
            .....|FJLJ|FJ|F7|.LJ\n\
            ....FJL-7.||.||||...\n\
            ....L---J.LJ.LJLJ...";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "8");
    }

    #[test]
    fn test_part_2_example_4() {
        let input = "FF7FSF7F7F7F7F7F---7\n\
            L|LJ||||||||||||F--J\n\
            FL-7LJLJ||||||LJL-77\n\
            F--JF--7||LJLJ7F7FJ-\n\
            L---JF-JLJ.||-FJLJJ7\n\
            |F|F-JF---7F7-L7L|7|\n\
            |FFJF7L7F-JF7|JL---7\n\
            7-L-JL7||F7|L7F-7F7|\n\
            L.L7LFJ|||||FJL7||LJ\n\
            L7JLJL-JLJLJL--JLJ.L";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "10");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/10")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "435");
    }
}
