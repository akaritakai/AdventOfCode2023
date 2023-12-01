use crate::input_fetcher::InputFetcher;
use crate::puzzle::Puzzle;

mod day01;
mod input_fetcher;
mod puzzle;

fn main() {
    let fetcher = InputFetcher::create();
    let puzzles: Vec<Box<dyn Puzzle>> =
        vec![day01::Day::create(fetcher.get_input(1).unwrap().as_str())];
    for puzzle in puzzles {
        println!("Day {:02} Part 1: {}", puzzle.day(), puzzle.solve_part_1());
        println!("Day {:02} Part 2: {}", puzzle.day(), puzzle.solve_part_2());
    }
}
