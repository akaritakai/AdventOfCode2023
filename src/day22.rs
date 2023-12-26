use crate::puzzle::Puzzle;
use lazy_regex::regex_captures;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let blocks = self.parse_blocks();
        let settled_blocks = settle_blocks(blocks);
        let graph = BlockGraph::new(settled_blocks);
        graph.count_safe_to_disintegrate().to_string()
    }

    fn solve_part_2(&self) -> String {
        let blocks = self.parse_blocks();
        let settled_blocks = settle_blocks(blocks);
        let graph = BlockGraph::new(settled_blocks);
        graph.count_falling_blocks().to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_blocks(&self) -> Vec<Block> {
        self.input
            .lines()
            .map(|line| {
                let (_, x0, y0, z0, x1, y1, z1) =
                    regex_captures!(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)", line).unwrap();
                let x0 = x0.parse::<i32>().unwrap();
                let y0 = y0.parse::<i32>().unwrap();
                let z0 = z0.parse::<i32>().unwrap();
                let x1 = x1.parse::<i32>().unwrap();
                let y1 = y1.parse::<i32>().unwrap();
                let z1 = z1.parse::<i32>().unwrap();
                Block {
                    x_min: x0.min(x1),
                    x_max: x0.max(x1),
                    y_min: y0.min(y1),
                    y_max: y0.max(y1),
                    z_min: z0.min(z1),
                    z_max: z0.max(z1),
                }
            })
            .collect()
    }
}

fn settle_blocks(mut blocks: Vec<Block>) -> Vec<Block> {
    blocks.sort_by_key(|block| block.z_min);
    let mut settled_blocks = Vec::new();
    for block in blocks {
        let mut z_max = 0;
        for settled_block in &settled_blocks {
            if block.xy_intersects(settled_block) {
                z_max = z_max.max(settled_block.z_max);
            }
        }
        settled_blocks.push(Block {
            x_min: block.x_min,
            x_max: block.x_max,
            y_min: block.y_min,
            y_max: block.y_max,
            z_min: z_max + 1,
            z_max: z_max + 1 + block.z_max - block.z_min,
        });
    }
    settled_blocks
}

struct Block {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl Block {
    fn xy_intersects(&self, other: &Block) -> bool {
        self.x_min <= other.x_max
            && self.y_min <= other.y_max
            && self.x_max >= other.x_min
            && self.y_max >= other.y_min
    }

    fn supports(&self, other: &Block) -> bool {
        self.xy_intersects(other) && self.z_max == other.z_min - 1
    }
}

struct BlockGraph {
    supports: Vec<Vec<usize>>,
    supported_by_counts: Vec<usize>,
}

impl BlockGraph {
    fn new(blocks: Vec<Block>) -> BlockGraph {
        let n = blocks.len();
        let mut supports = vec![Vec::new(); n];
        let mut supported_by_counts = vec![0; n];

        for i in 0..n {
            for j in i + 1..n {
                if blocks[i].supports(&blocks[j]) {
                    supports[i].push(j);
                    supported_by_counts[j] += 1;
                }
            }
        }

        BlockGraph {
            supports,
            supported_by_counts,
        }
    }

    fn count_safe_to_disintegrate(&self) -> usize {
        self.supports
            .iter()
            .filter(|supportees| supportees.iter().all(|&j| self.supported_by_counts[j] > 1))
            .count()
    }

    fn count_falling_blocks(&self) -> usize {
        let mut sum = 0;
        let n = self.supports.len();
        for i in 0..n {
            let mut count = 0;
            let mut supported_by_counts = self.supported_by_counts.clone();
            let mut queue = vec![i];
            while let Some(i) = queue.pop() {
                count += 1;
                for j in &self.supports[i] {
                    supported_by_counts[*j] -= 1;
                    if supported_by_counts[*j] == 0 {
                        queue.push(*j);
                    }
                }
            }
            sum += count - 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "1,0,1~1,2,1\n\
            0,0,2~2,0,2\n\
            0,2,3~2,2,3\n\
            0,0,4~0,2,4\n\
            2,0,5~2,2,5\n\
            0,1,6~2,1,6\n\
            1,1,8~1,1,9";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "5");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/22")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "432");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "1,0,1~1,2,1\n\
            0,0,2~2,0,2\n\
            0,2,3~2,2,3\n\
            0,0,4~0,2,4\n\
            2,0,5~2,2,5\n\
            0,1,6~2,1,6\n\
            1,1,8~1,1,9";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "7");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/22")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "63166");
    }
}
