use crate::Solutions;
use lib_aoc::prelude::*;

impl Solution<DAY_01> for Solutions {
    type Input<'i> = Vec<u64>;
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .lines()
            .map(str::parse::<u64>)
            .map(Result::unwrap)
            .collect::<Vec<_>>()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        input.iter().sum::<u64>().into()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        input.iter().map(|x| x.pow(2)).sum::<u64>().into()
    }
}
