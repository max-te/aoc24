use lib_aoc::prelude::*;
struct Solutions {}

mod day01;
mod day02;
mod day03;
mod day04;
mod util;

impl Solver for Solutions {
    fn load(day: u8) -> String {
        std::fs::read_to_string(format!("src/day{day:02}/input.txt"))
            .expect("Puzzle input could not be read.")
    }

    fn load_test(day: u8, part: bool) -> String {
        if part {
            if let Ok(puzzle) = std::fs::read_to_string(format!("src/day{day:02}/test_2.txt")) {
                return puzzle;
            }
        }
        std::fs::read_to_string(format!("src/day{day:02}/test.txt"))
            .expect("Puzzle input could not be read.")
    }
}

fn main() {
    solve_through!(Solutions, 4);
}
