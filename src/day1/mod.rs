use std::{collections::HashMap, iter::zip};

use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<u64>, Vec<u64>);
type Output = u64;

#[aoc_generator(day1)]
pub fn parse(puzzle: &str) -> Input {
    puzzle
        .lines()
        .map(|s| {
            s.split_once("   ")
                .expect("input lines must all have two numbers")
        })
        .map(|(a, b)| {
            (
                str::parse::<u64>(a).expect("should be a number"),
                str::parse::<u64>(b).expect("should be a number"),
            )
        })
        .collect::<(Vec<_>, Vec<_>)>()
}

#[aoc(day1, part1)]
fn part_one(input: &Input) -> Output {
    let mut left_list = input.0.clone();
    left_list.sort();
    let mut right_list = input.1.clone();
    right_list.sort();
    zip(left_list, right_list).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn part1(puzzle: &str) -> Output {
    part_one(&parse(puzzle))
}

#[aoc(day1, part2)]
fn part_two(input: &Input) -> Output {
    let mut counts: HashMap<&u64, u64> = HashMap::with_capacity(input.1.len());
    for number in input.1.iter() {
        *counts.entry(number).or_default() += 1
    }
    input
        .0
        .iter()
        .map(|number| number * counts.get(number).unwrap_or(&0))
        .sum()
}

pub fn part2(puzzle: &str) -> Output {
    part_two(&parse(puzzle))
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, 11);
    }

    #[test]
    fn example2() {
        let res = part2(include_str!("test.txt"));
        assert_eq!(res, 31);
    }
}
