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
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
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
        day14::Day::create(fetcher.get_input(14).unwrap().as_str()),
        day15::Day::create(fetcher.get_input(15).unwrap().as_str()),
        day16::Day::create(fetcher.get_input(16).unwrap().as_str()),
        day17::Day::create(fetcher.get_input(17).unwrap().as_str()),
        day18::Day::create(fetcher.get_input(18).unwrap().as_str()),
        day19::Day::create(fetcher.get_input(19).unwrap().as_str()),
        day20::Day::create(fetcher.get_input(20).unwrap().as_str()),
        day21::Day::create(fetcher.get_input(21).unwrap().as_str()),
        day22::Day::create(fetcher.get_input(22).unwrap().as_str()),
        day23::Day::create(fetcher.get_input(23).unwrap().as_str()),
        day24::Day::create(fetcher.get_input(24).unwrap().as_str()),
        day25::Day::create(fetcher.get_input(25).unwrap().as_str()),
    ];
    for (i, puzzle) in puzzles.iter().enumerate() {
        println!("Day {:02} Part 1: {}", i + 1, puzzle.solve_part_1());
        println!("Day {:02} Part 2: {}", i + 1, puzzle.solve_part_2());
    }
}
