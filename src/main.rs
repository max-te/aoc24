use lib_aoc::prelude::*;
struct Solutions {}

mod days;
mod util;

impl Solver for Solutions {
    fn load(day: u8) -> String {
        std::fs::read_to_string(format!("src/inputs/{day:02}.txt"))
            .expect("Puzzle input could not be read.")
    }

    fn load_test(day: u8, part: bool) -> String {
        if part {
            if let Ok(puzzle) = std::fs::read_to_string(format!("src/inputs/test_{day:02}_2.txt")) {
                return puzzle;
            }
        }
        std::fs::read_to_string(format!("src/inputs/test_{day:02}.txt"))
            .expect("Puzzle input could not be read.")
    }
}

fn main() {
    solve_through!(Solutions, 3);
}
