use crate::puzzle::Puzzle;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn day(&self) -> u8 {
        6
    }

    fn solve_part_1(&self) -> String {
        let mut lines = self.input.lines();
        let times = parse_line_to_numbers(lines.next().unwrap());
        let distances = parse_line_to_numbers(lines.next().unwrap());
        let mut result = 1;
        for (time, record) in times.iter().zip(distances) {
            result *= calculate_ways_to_win(*time, record);
        }
        result.to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut lines = self.input.lines();
        let time = extract_single_number_from_line(lines.next().unwrap());
        let distance = extract_single_number_from_line(lines.next().unwrap());
        calculate_ways_to_win(time, distance).to_string()
    }
}

fn calculate_ways_to_win(time: u64, distance: u64) -> u64 {
    let discriminant = (time * time - 4 * distance) as f64;
    let sqrt_discriminant = discriminant.sqrt();
    let mut min_hold_time = (((time as f64) - sqrt_discriminant) / 2.0).ceil() as u64;
    let mut max_hold_time = (((time as f64) + sqrt_discriminant) / 2.0).floor() as u64;
    if sqrt_discriminant.fract() == 0.0 {
        min_hold_time += 1;
        max_hold_time -= 1;
    }
    max_hold_time - min_hold_time + 1
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }
}

fn parse_line_to_numbers(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>()
}

fn extract_single_number_from_line(line: &str) -> u64 {
    line.chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "288");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/06")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "275724");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "71503");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/06")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "37286485");
    }
}
