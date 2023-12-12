use crate::puzzle::Puzzle;
use num::integer::Roots;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        solve_generic(&self.input, 2).to_string()
    }

    fn solve_part_2(&self) -> String {
        solve_generic(&self.input, 1_000_000).to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }
}

fn solve_generic(input: &str, gap_factor: usize) -> usize {
    let size = input.len().sqrt();
    let (mut x_counts, mut y_counts) = (vec![0; size], vec![0; size]);
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                x_counts[x] += 1;
                y_counts[y] += 1;
            }
        }
    }
    dist(x_counts, gap_factor) + dist(y_counts, gap_factor)
}

fn dist(galaxy_counts: Vec<usize>, gap_factor: usize) -> usize {
    let mut gap_count = 0;
    let mut galaxies_seen = 0;
    let mut total_distance = 0;
    let mut weighted_position_sum = 0;
    for (i, &galaxies) in galaxy_counts.iter().enumerate() {
        if galaxies == 0 {
            gap_count += 1;
            continue;
        }
        let position = i + gap_count * (gap_factor - 1);
        total_distance += galaxies * galaxies_seen * position;
        total_distance -= galaxies * weighted_position_sum;
        galaxies_seen += galaxies;
        weighted_position_sum += galaxies * position;
    }
    total_distance
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "...#......\n\
            .......#..\n\
            #.........\n\
            ..........\n\
            ......#...\n\
            .#........\n\
            .........#\n\
            ..........\n\
            .......#..\n\
            #...#.....";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "374");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/11")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "9957702");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "...#......\n\
            .......#..\n\
            #.........\n\
            ..........\n\
            ......#...\n\
            .#........\n\
            .........#\n\
            ..........\n\
            .......#..\n\
            #...#.....";
        assert_eq!(solve_generic(input, 10), 1030);
    }

    #[test]
    fn test_part_2_example_2() {
        let input = "...#......\n\
            .......#..\n\
            #.........\n\
            ..........\n\
            ......#...\n\
            .#........\n\
            .........#\n\
            ..........\n\
            .......#..\n\
            #...#.....";
        assert_eq!(solve_generic(input, 100), 8410);
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/11")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "512240933238");
    }
}
