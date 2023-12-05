use crate::puzzle::Puzzle;
use lazy_regex::regex;
use rangemap::RangeMap;
use std::ops::Range;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn day(&self) -> u8 {
        5
    }

    fn solve_part_1(&self) -> String {
        let mut seeds = Vec::new();
        let seed_line = self.input.lines().next().unwrap();
        for m in regex!(r"\d+").find_iter(seed_line) {
            let seed = m.as_str().parse::<i64>().unwrap();
            seeds.push(seed..seed + 1);
        }
        for map in self.parse_maps() {
            seeds = map_range(&mut seeds, &map);
        }
        seeds
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut seeds = Vec::new();
        let seed_line = self.input.lines().next().unwrap();
        for cap in regex!(r"(\d+) (\d+)").captures_iter(seed_line) {
            let start = cap[1].parse::<i64>().unwrap();
            let length = cap[2].parse::<i64>().unwrap();
            seeds.push(start..start + length);
        }
        for map in self.parse_maps() {
            seeds = map_range(&mut seeds, &map);
        }
        seeds
            .iter()
            .map(|range| range.start)
            .min()
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

    fn parse_maps(&self) -> Vec<RangeMap<i64, i64>> {
        let mut maps = Vec::new();
        for block in self.input.split("\n\n").skip(1) {
            let mut map = RangeMap::new();
            for line in block.lines().skip(1) {
                let mut parts = line.split_whitespace();
                let dst = parts.next().unwrap().parse::<i64>().unwrap();
                let src = parts.next().unwrap().parse::<i64>().unwrap();
                let length = parts.next().unwrap().parse::<i64>().unwrap();
                map.insert(src..src + length, dst - src);
            }
            maps.push(map);
        }
        maps
    }
}

fn map_range(inputs: &mut Vec<Range<i64>>, map: &RangeMap<i64, i64>) -> Vec<Range<i64>> {
    let mut output = Vec::new();
    while let Some(input) = inputs.pop() {
        if map.overlaps(&input) {
            for (range, offset) in map.overlapping(&input) {
                let start = std::cmp::max(input.start, range.start);
                let end = std::cmp::min(input.end, range.end);
                output.push(start + offset..end + offset);
                if input.start < start {
                    inputs.push(input.start..start);
                }
                if end < input.end {
                    inputs.push(end..input.end);
                }
            }
        } else {
            output.push(input);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "seeds: 79 14 55 13\n\
            \n\
            seed-to-soil map:\n\
            50 98 2\n\
            52 50 48\n\
            \n\
            soil-to-fertilizer map:\n\
            0 15 37\n\
            37 52 2\n\
            39 0 15\n\
            \n\
            fertilizer-to-water map:\n\
            49 53 8\n\
            0 11 42\n\
            42 0 7\n\
            57 7 4\n\
            \n\
            water-to-light map:\n\
            88 18 7\n\
            18 25 70\n\
            \n\
            light-to-temperature map:\n\
            45 77 23\n\
            81 45 19\n\
            68 64 13\n\
            \n\
            temperature-to-humidity map:\n\
            0 69 1\n\
            1 0 69\n\
            \n\
            humidity-to-location map:\n\
            60 56 37\n\
            56 93 4";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "35");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/05")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1181555926");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "seeds: 79 14 55 13\n\
            \n\
            seed-to-soil map:\n\
            50 98 2\n\
            52 50 48\n\
            \n\
            soil-to-fertilizer map:\n\
            0 15 37\n\
            37 52 2\n\
            39 0 15\n\
            \n\
            fertilizer-to-water map:\n\
            49 53 8\n\
            0 11 42\n\
            42 0 7\n\
            57 7 4\n\
            \n\
            water-to-light map:\n\
            88 18 7\n\
            18 25 70\n\
            \n\
            light-to-temperature map:\n\
            45 77 23\n\
            81 45 19\n\
            68 64 13\n\
            \n\
            temperature-to-humidity map:\n\
            0 69 1\n\
            1 0 69\n\
            \n\
            humidity-to-location map:\n\
            60 56 37\n\
            56 93 4";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "46");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/05")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "37806486");
    }
}
