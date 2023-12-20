use crate::puzzle::Puzzle;
use lazy_regex::regex_captures;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let plan = self.parse_part_1();
        solve_generic(plan).to_string()
    }

    fn solve_part_2(&self) -> String {
        let plan = self.parse_part_2();
        solve_generic(plan).to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_part_1(&self) -> Vec<(Point, u32)> {
        self.input
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let dir = match parts.next().unwrap() {
                    "U" => UP,
                    "D" => DOWN,
                    "L" => LEFT,
                    "R" => RIGHT,
                    _ => unreachable!(),
                };
                let dist = parts.next().unwrap().parse::<u32>().unwrap();
                (dir, dist)
            })
            .collect()
    }

    fn parse_part_2(&self) -> Vec<(Point, u32)> {
        self.input
            .lines()
            .map(|line| {
                let (_, dist, dir) = regex_captures!(r".* \(#([a-f0-9]{5})(\d)\)", line).unwrap();
                let dir = match dir {
                    "0" => RIGHT,
                    "1" => DOWN,
                    "2" => LEFT,
                    "3" => UP,
                    _ => unreachable!(),
                };
                let dist = u32::from_str_radix(dist, 16).unwrap();
                (dir, dist)
            })
            .collect()
    }
}

fn solve_generic(plan: Vec<(Point, u32)>) -> u64 {
    let mut boundary = Point { row: 0, col: 0 };
    let mut perimeter = 0u64;
    let mut area = 0i128;
    for (dir, dist) in plan {
        let position = boundary + dir * dist;
        area += boundary.row * position.col - boundary.col * position.row;
        perimeter += dist as u64;
        boundary = position;
    }
    (area.unsigned_abs() as u64 - perimeter) / 2 + perimeter + 1
}

const UP: Point = Point { row: -1, col: 0 };
const DOWN: Point = Point { row: 1, col: 0 };
const LEFT: Point = Point { row: 0, col: 1 };
const RIGHT: Point = Point { row: 0, col: -1 };

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    row: i128,
    col: i128,
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

impl std::ops::Mul<u32> for Point {
    type Output = Point;
    fn mul(self, other: u32) -> Point {
        Point {
            row: self.row * other as i128,
            col: self.col * other as i128,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "R 6 (#70c710)\n\
            D 5 (#0dc571)\n\
            L 2 (#5713f0)\n\
            D 2 (#d2c081)\n\
            R 2 (#59c680)\n\
            D 2 (#411b91)\n\
            L 5 (#8ceee2)\n\
            U 2 (#caa173)\n\
            L 1 (#1b58a2)\n\
            U 2 (#caa171)\n\
            R 2 (#7807d2)\n\
            U 3 (#a77fa3)\n\
            L 2 (#015232)\n\
            U 2 (#7a21e3)";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "62");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/18")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "47139");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "R 6 (#70c710)\n\
            D 5 (#0dc571)\n\
            L 2 (#5713f0)\n\
            D 2 (#d2c081)\n\
            R 2 (#59c680)\n\
            D 2 (#411b91)\n\
            L 5 (#8ceee2)\n\
            U 2 (#caa173)\n\
            L 1 (#1b58a2)\n\
            U 2 (#caa171)\n\
            R 2 (#7807d2)\n\
            U 3 (#a77fa3)\n\
            L 2 (#015232)\n\
            U 2 (#7a21e3)";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "952408144115");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/18")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "173152345887206");
    }
}
