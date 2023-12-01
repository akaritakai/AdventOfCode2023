pub trait Puzzle {
    fn day(&self) -> u8;
    fn solve_part_1(&self) -> String;
    fn solve_part_2(&self) -> String;
}
