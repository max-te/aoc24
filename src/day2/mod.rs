use aoc_runner_derive::{aoc, aoc_generator};
use crate::util::VecVec;

type Input = VecVec<u64>;
type Output = u64;

#[aoc_generator(day2)]
fn parse(puzzle: &str) -> Input {
    let mut input = VecVec::with_capacity(puzzle.len() / 16);
    for line in puzzle.lines() {
        input.push_from(line.split(' ').map(str::parse::<u64>).map(Result::unwrap));
    }
    input
}

#[aoc(day2, part1)]
fn part_one(input: &Input) -> Output {
    input.iter().filter(|seq| sequence_is_safe(seq)).count() as u64
}

pub fn part1(puzzle: &str) -> Output {
    part_one(&parse(puzzle))
}

#[aoc(day2, part2)]
fn part_two(input: &Input) -> Output {
    input
        .iter()
        .filter(|seq| sequence_is_safe(seq) || problem_dampable(seq))
        .count() as u64
}

pub fn part2(puzzle: &str) -> Output {
    part_two(&parse(puzzle))
}

fn sequence_is_safe(seq: &[u64]) -> bool {
    let mut iter = seq.iter().peekable();
    let mut prev = iter.next().unwrap();
    let is_decreasing = *iter.peek().unwrap() < prev;
    for i in iter {
        if is_decreasing == (i > prev) {
            // eprintln!("{seq:?} is non-monotonic at {prev}, {i}");
            return false;
        }
        let step = i.abs_diff(*prev);
        if !(1..=3).contains(&step) {
            // eprintln!("{seq:?} has a step of {step} at {prev}, {i}");
            return false;
        }
        prev = i;
    }
    // eprintln!("{seq:?} is safe");
    true
}

fn problem_dampable(seq: &[u64]) -> bool {
    for i in 0..seq.len() {
        let mut dampened = seq.to_vec();
        dampened.remove(i);
        if sequence_is_safe(&dampened) {
            // eprintln!("{seq:?} is dampable at {i}");
            return true;
        }
    }
    false
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, 2);
    }

    #[test]
    fn example2() {
        let res = part2(include_str!("test.txt"));
        assert_eq!(res, 4);
    }
}
