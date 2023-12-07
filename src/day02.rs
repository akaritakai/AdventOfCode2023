use crate::puzzle::Puzzle;
use lazy_regex::regex;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        self.input
            .lines()
            .map(parse_game)
            .enumerate()
            .filter_map(|(i, game)| {
                if game
                    .iter()
                    .all(|reveal| reveal.red <= 12 && reveal.green <= 13 && reveal.blue <= 14)
                {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.input
            .lines()
            .map(parse_game)
            .map(|game| {
                game.iter()
                    .fold((0, 0, 0), |(max_red, max_green, max_blue), reveal| {
                        (
                            std::cmp::max(max_red, reveal.red),
                            std::cmp::max(max_green, reveal.green),
                            std::cmp::max(max_blue, reveal.blue),
                        )
                    })
            })
            .map(|(max_red, max_green, max_blue)| max_red * max_green * max_blue)
            .sum::<i32>()
            .to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }
}

struct Colors {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_game(line: &str) -> Vec<Colors> {
    line.split(';')
        .map(|reveal| {
            let re = regex!(r"(\d+) (red|green|blue)");
            let (mut red, mut green, mut blue) = (0, 0, 0);
            for cap in re.captures_iter(reveal) {
                let count = cap[1].parse::<i32>().unwrap();
                match &cap[2] {
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => unreachable!(),
                }
            }
            Colors { red, green, blue }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "8");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/02")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1734");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "2286");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/02")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "70387");
    }
}
