use crate::puzzle::Puzzle;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        least_heat_loss::<1, 3>(&self.parse_grid()).to_string()
    }

    fn solve_part_2(&self) -> String {
        least_heat_loss::<4, 10>(&self.parse_grid()).to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_grid(&self) -> Vec<Vec<usize>> {
        self.input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>()
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
struct LavaFlow {
    loc: (isize, isize),
    dir: (isize, isize),
    count: usize,
}

fn least_heat_loss<const MIN_STEPS: usize, const MAX_STEPS: usize>(grid: &[Vec<usize>]) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    let start = (0, 0);
    let end = (rows as isize - 1, cols as isize - 1);

    let mut score = HashMap::new();

    let mut queue = BinaryHeap::new();
    queue.push((
        Reverse((0, 0)),
        LavaFlow {
            loc: start,
            dir: (0, 1),
            count: 0,
        },
    ));
    queue.push((
        Reverse((0, 0)),
        LavaFlow {
            loc: start,
            dir: (1, 0),
            count: 0,
        },
    ));

    while let Some((Reverse((_, cost)), flow)) = queue.pop() {
        if flow.loc == end && flow.count >= MIN_STEPS {
            return cost;
        }
        for new_flow in neighbors::<MIN_STEPS, MAX_STEPS>(rows, cols, flow) {
            let new_cost = cost + grid[new_flow.loc.0 as usize][new_flow.loc.1 as usize];
            if new_cost < score.get(&new_flow).copied().unwrap_or(usize::MAX) {
                let new_heuristic_cost = new_cost
                    + (end.0 - new_flow.loc.0).unsigned_abs()
                    + (end.1 - new_flow.loc.1).unsigned_abs();
                score.insert(new_flow, new_cost);
                queue.push((Reverse((new_heuristic_cost, new_cost)), new_flow));
            }
        }
    }

    unreachable!()
}

fn neighbors<const MIN_STEPS: usize, const MAX_STEPS: usize>(
    rows: usize,
    cols: usize,
    flow: LavaFlow,
) -> impl Iterator<Item = LavaFlow> {
    let (row, col, drow, dcol, count) =
        (flow.loc.0, flow.loc.1, flow.dir.0, flow.dir.1, flow.count);
    let in_bounds = move |r, c| r >= 0 && r < rows as isize && c >= 0 && c < cols as isize;
    [(drow, dcol), (dcol, -drow), (-dcol, drow)]
        .into_iter()
        .filter_map(move |(new_drow, new_dcol)| {
            let is_turn = new_drow != drow || new_dcol != dcol;
            let (new_row, new_col) = (row + new_drow, col + new_dcol);
            if (is_turn && count < MIN_STEPS)
                || (!is_turn && count == MAX_STEPS)
                || !in_bounds(new_row, new_col)
            {
                None
            } else {
                Some(LavaFlow {
                    loc: (new_row, new_col),
                    dir: (new_drow, new_dcol),
                    count: if is_turn { 1 } else { count + 1 },
                })
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "2413432311323\n\
            3215453535623\n\
            3255245654254\n\
            3446585845452\n\
            4546657867536\n\
            1438598798454\n\
            4457876987766\n\
            3637877979653\n\
            4654967986887\n\
            4564679986453\n\
            1224686865563\n\
            2546548887735\n\
            4322674655533";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "102");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/17")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "758");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "2413432311323\n\
            3215453535623\n\
            3255245654254\n\
            3446585845452\n\
            4546657867536\n\
            1438598798454\n\
            4457876987766\n\
            3637877979653\n\
            4654967986887\n\
            4564679986453\n\
            1224686865563\n\
            2546548887735\n\
            4322674655533";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "94");
    }

    #[test]
    fn test_part_2_example_2() {
        let input = "111111111111\n\
            999999999991\n\
            999999999991\n\
            999999999991\n\
            999999999991";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "71");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/17")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "892");
    }
}
