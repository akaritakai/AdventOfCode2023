use crate::input_fetcher::InputFetcher;
use crate::puzzle::Puzzle;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
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
        day06::Day::create(fetcher.get_input(6).unwrap().as_str()),
        day07::Day::create(fetcher.get_input(7).unwrap().as_str()),
        day08::Day::create(fetcher.get_input(8).unwrap().as_str()),
        day09::Day::create(fetcher.get_input(9).unwrap().as_str()),
        day10::Day::create(fetcher.get_input(10).unwrap().as_str()),
        day11::Day::create(fetcher.get_input(11).unwrap().as_str()),
        day12::Day::create(fetcher.get_input(12).unwrap().as_str()),
        day13::Day::create(fetcher.get_input(13).unwrap().as_str()),
    ];
    for (i, puzzle) in puzzles.iter().enumerate() {
        println!("Day {:02} Part 1: {}", i + 1, puzzle.solve_part_1());
        println!("Day {:02} Part 2: {}", i + 1, puzzle.solve_part_2());
    }
}
