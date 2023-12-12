use crate::puzzle::Puzzle;
use lazy_regex::regex_captures;
use std::collections::HashMap;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        self.parse_network()
            .num_steps("AAA", |node| node == "ZZZ")
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let network = self.parse_network();
        network
            .graph
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|key| network.num_steps(key, |node| node.ends_with('Z')))
            .fold(1, num::integer::lcm)
            .to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_network(&self) -> Network {
        let mut sections = self.input.split("\n\n");
        let instructions = sections.next().unwrap().to_string();
        let mut graph = HashMap::new();
        for line in sections.next().unwrap().lines() {
            let (_, node, left, right) =
                regex_captures!(r"(\S+) = \((\S+), (\S+)\)", line).unwrap();
            graph.insert(node.to_string(), (left.to_string(), right.to_string()));
        }
        Network {
            instructions,
            graph,
        }
    }
}

struct Network {
    instructions: String,
    graph: HashMap<String, (String, String)>,
}

impl Network {
    fn num_steps<F>(&self, start: &str, is_end: F) -> usize
    where
        F: Fn(&str) -> bool,
    {
        let mut node = start;
        let mut depth = 0;
        for instruction in self.instructions.chars().cycle() {
            let (left, right) = self.graph.get(node).unwrap();
            node = match instruction {
                'L' => left,
                'R' => right,
                _ => unreachable!(),
            };
            depth += 1;
            if is_end(node) {
                break;
            }
        }
        depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "RL\n\
            \n\
            AAA = (BBB, CCC)\n\
            BBB = (DDD, EEE)\n\
            CCC = (ZZZ, GGG)\n\
            DDD = (DDD, DDD)\n\
            EEE = (EEE, EEE)\n\
            GGG = (GGG, GGG)\n\
            ZZZ = (ZZZ, ZZZ)";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "2");
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "LLR\n\
            \n\
            AAA = (BBB, BBB)\n\
            BBB = (AAA, ZZZ)\n\
            ZZZ = (ZZZ, ZZZ)";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "6");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/08")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "14257");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "LR\n\
            \n\
            11A = (11B, XXX)\n\
            11B = (XXX, 11Z)\n\
            11Z = (11B, XXX)\n\
            22A = (22B, XXX)\n\
            22B = (22C, 22C)\n\
            22C = (22Z, 22Z)\n\
            22Z = (22B, 22B)\n\
            XXX = (XXX, XXX)";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "6");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/08")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "16187743689077");
    }
}
