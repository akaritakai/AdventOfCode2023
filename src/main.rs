use crate::input_fetcher::InputFetcher;
use crate::puzzle::Puzzle;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod input_fetcher;
mod puzzle;

fn main() {
    let fetcher = InputFetcher::create();
    let puzzles: Vec<Box<dyn Puzzle>> = vec![
        day01::Day::create(fetcher.get_input(1).unwrap().as_str()),
        day02::Day::create(fetcher.get_input(2).unwrap().as_str()),
        day03::Day::create(fetcher.get_input(3).unwrap().as_str()),
        day04::Day::create(fetcher.get_input(4).unwrap().as_str()),
        day05::Day::create(fetcher.get_input(5).unwrap().as_str()),
    ];
    for puzzle in puzzles {
        println!("Day {:02} Part 1: {}", puzzle.day(), puzzle.solve_part_1());
        println!("Day {:02} Part 2: {}", puzzle.day(), puzzle.solve_part_2());
    }
}
