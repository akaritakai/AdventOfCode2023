use crate::puzzle::Puzzle;
use lazy_regex::regex_captures;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let workflows = self.parse_workflows();
        let intervals = find_accepted_intervals(&workflows);
        self.parse_parts()
            .iter()
            .filter_map(|part| {
                if intervals.contains(part) {
                    Some(part.0 + part.1 + part.2 + part.3)
                } else {
                    None
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let workflows = self.parse_workflows();
        let intervals = find_accepted_intervals(&workflows);
        intervals.size().to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_workflows(&self) -> HashMap<String, Vec<Rule>> {
        let mut workflows = HashMap::new();
        for line in self.input.split("\n\n").next().unwrap().lines() {
            let (_, name, rules_str) = regex_captures!(r"(\S+)\{(\S+)\}", line).unwrap();
            let mut rules = Vec::new();
            for rule_str in rules_str.split(',') {
                if let Some((_, var, op, value, dst)) =
                    regex_captures!(r"(x|m|a|s)(<|>)(\d+):(\S+)", rule_str)
                {
                    let var = var.chars().next().unwrap();
                    let op = op.chars().next().unwrap();
                    let value = value.parse().unwrap();
                    rules.push(Rule::If(var, op, value, dst.to_string()));
                } else {
                    rules.push(Rule::Goto(rule_str.to_string()));
                }
            }
            workflows.insert(name.to_string(), rules);
        }
        workflows
    }

    fn parse_parts(&self) -> Vec<Part> {
        self.input
            .split("\n\n")
            .nth(1)
            .unwrap()
            .lines()
            .map(|line| {
                let (_, x, m, a, s) =
                    regex_captures!(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}", line).unwrap();
                (
                    x.parse().unwrap(),
                    m.parse().unwrap(),
                    a.parse().unwrap(),
                    s.parse().unwrap(),
                )
            })
            .collect()
    }
}

fn find_accepted_intervals(workflows: &HashMap<String, Vec<Rule>>) -> IntervalSet {
    let mut seen = HashSet::new();
    let mut intervals = Vec::new();
    find_accepted_intervals_impl(
        "in".to_string(),
        &(1..4001, 1..4001, 1..4001, 1..4001),
        workflows,
        &mut seen,
        &mut intervals,
    );
    IntervalSet::new(intervals)
}

fn find_accepted_intervals_impl(
    node: String,
    interval: &Interval,
    workflows: &HashMap<String, Vec<Rule>>,
    seen_states: &mut HashSet<String>,
    accepted_intervals: &mut Vec<Interval>,
) {
    if seen_states.contains(&node)
        || node == "R"
        || interval.0.is_empty()
        || interval.1.is_empty()
        || interval.2.is_empty()
        || interval.3.is_empty()
    {
        return;
    }
    if node == "A" {
        accepted_intervals.push(interval.clone());
        return;
    }
    seen_states.insert(node.clone());
    let mut current_interval = interval.clone();
    for rule in workflows.get(&node).unwrap() {
        match rule {
            Rule::If(var, op, value, dst) => {
                let mut next_interval = current_interval.clone();
                match (var, op) {
                    ('x', '<') => {
                        next_interval.0 = *value..current_interval.0.end;
                        current_interval.0 = current_interval.0.start..*value;
                    }
                    ('x', '>') => {
                        next_interval.0 = current_interval.0.start..*value + 1;
                        current_interval.0 = *value + 1..current_interval.0.end;
                    }
                    ('m', '<') => {
                        next_interval.1 = *value..current_interval.1.end;
                        current_interval.1 = current_interval.1.start..*value;
                    }
                    ('m', '>') => {
                        next_interval.1 = current_interval.1.start..*value + 1;
                        current_interval.1 = *value + 1..current_interval.1.end;
                    }
                    ('a', '<') => {
                        next_interval.2 = *value..current_interval.2.end;
                        current_interval.2 = current_interval.2.start..*value;
                    }
                    ('a', '>') => {
                        next_interval.2 = current_interval.2.start..*value + 1;
                        current_interval.2 = *value + 1..current_interval.2.end;
                    }
                    ('s', '<') => {
                        next_interval.3 = *value..current_interval.3.end;
                        current_interval.3 = current_interval.3.start..*value;
                    }
                    ('s', '>') => {
                        next_interval.3 = current_interval.3.start..*value + 1;
                        current_interval.3 = *value + 1..current_interval.3.end;
                    }
                    _ => unreachable!(),
                }
                find_accepted_intervals_impl(
                    dst.clone(),
                    &current_interval,
                    workflows,
                    seen_states,
                    accepted_intervals,
                );
                current_interval = next_interval;
            }
            Rule::Goto(dst) => {
                find_accepted_intervals_impl(
                    dst.clone(),
                    &current_interval,
                    workflows,
                    seen_states,
                    accepted_intervals,
                );
            }
        }
    }
}

enum Rule {
    If(char, char, usize, String),
    Goto(String),
}

type Part = (usize, usize, usize, usize);
type Interval = (Range<usize>, Range<usize>, Range<usize>, Range<usize>);

struct IntervalSet {
    set: Vec<Interval>,
}

impl IntervalSet {
    fn new(intervals: Vec<Interval>) -> Self {
        IntervalSet { set: intervals }
    }

    fn contains(&self, part: &Part) -> bool {
        self.set.iter().any(|interval| {
            interval.0.contains(&part.0)
                && interval.1.contains(&part.1)
                && interval.2.contains(&part.2)
                && interval.3.contains(&part.3)
        })
    }

    fn size(&self) -> usize {
        self.set
            .iter()
            .map(|interval| {
                (interval.0.end - interval.0.start)
                    * (interval.1.end - interval.1.start)
                    * (interval.2.end - interval.2.start)
                    * (interval.3.end - interval.3.start)
            })
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}\n\
            pv{a>1716:R,A}\n\
            lnx{m>1548:A,A}\n\
            rfg{s<537:gd,x>2440:R,A}\n\
            qs{s>3448:A,lnx}\n\
            qkq{x<1416:A,crn}\n\
            crn{x>2662:A,R}\n\
            in{s<1351:px,qqz}\n\
            qqz{s>2770:qs,m<1801:hdj,R}\n\
            gd{a>3333:R,R}\n\
            hdj{m>838:A,pv}\n\
            \n\
            {x=787,m=2655,a=1222,s=2876}\n\
            {x=1679,m=44,a=2067,s=496}\n\
            {x=2036,m=264,a=79,s=2244}\n\
            {x=2461,m=1339,a=466,s=291}\n\
            {x=2127,m=1623,a=2188,s=1013}";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "19114");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/19")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "330820");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}\n\
            pv{a>1716:R,A}\n\
            lnx{m>1548:A,A}\n\
            rfg{s<537:gd,x>2440:R,A}\n\
            qs{s>3448:A,lnx}\n\
            qkq{x<1416:A,crn}\n\
            crn{x>2662:A,R}\n\
            in{s<1351:px,qqz}\n\
            qqz{s>2770:qs,m<1801:hdj,R}\n\
            gd{a>3333:R,R}\n\
            hdj{m>838:A,pv}\n\
            \n\
            {x=787,m=2655,a=1222,s=2876}\n\
            {x=1679,m=44,a=2067,s=496}\n\
            {x=2036,m=264,a=79,s=2244}\n\
            {x=2461,m=1339,a=466,s=291}\n\
            {x=2127,m=1623,a=2188,s=1013}";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "167409079868000");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/19")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "123972546935551");
    }
}
