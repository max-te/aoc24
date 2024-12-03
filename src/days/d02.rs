
use crate::Solutions;
use lib_aoc::prelude::*;

impl Solution<DAY_02> for Solutions {
    type Input<'i> = Vec<Vec<u64>>;
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .lines()
            .map(|s| {
                s.split(" ")
                    .map(str::parse::<u64>)
                    .collect::<Result<Vec<_>, _>>().unwrap()
            })
            .collect::<Vec<_>>()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        input.iter().filter(|seq| sequence_is_safe(seq)).count() as u64
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        input.iter()
            .filter(|seq| sequence_is_safe(seq) || problem_dampable(seq))
            .count() as u64
    }
}

fn sequence_is_safe(seq: &Vec<u64>) -> bool {
    let mut iter = seq.iter().peekable();
    let mut prev = iter.next().unwrap();
    let is_decreasing = *iter.peek().unwrap() < prev;
    for i in iter {
        if is_decreasing == (i > prev) {
            return false;
        }
        let step = i.abs_diff(*prev);
        if step < 1 || step > 3 {
            return false;
        }
        prev = i;
    }
    true
}

fn problem_dampable(seq: &Vec<u64>) -> bool {
    for i in 0..seq.len() {
        let mut dampened = seq.clone();
        dampened.remove(i);
        if sequence_is_safe(&dampened) {
            return true;
        }
    }
    false
}


impl Test<DAY_02> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 2,
            PART_TWO => 4,
        }
    }
}

derive_tests!(Solutions, DAY_02);
