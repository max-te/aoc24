use std::{collections::HashMap, iter::zip};

use crate::Solutions;
use lib_aoc::prelude::*;

impl Solution<DAY_01> for Solutions {
    type Input<'i> = (Vec<u64>, Vec<u64>);
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
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

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        let mut left_list = input.0.clone();
        left_list.sort();
        let mut right_list = input.1.clone();
        right_list.sort();
        zip(left_list, right_list).map(|(a, b)| a.abs_diff(b)).sum()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
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
}

impl Test<DAY_01> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 11,
            PART_TWO => 31,
        }
    }
}

derive_tests!(Solutions, DAY_01);
