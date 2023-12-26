use crate::puzzle::Puzzle;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        "TODO".to_string()
    }

    fn solve_part_2(&self) -> String {
        "TODO".to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "TODO";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "TODO");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/23")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "TODO");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "TODO";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "TODO");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/23")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "TODO");
    }
}
