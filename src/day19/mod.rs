use core::str;

use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

type Towel = ArrayVec<u8, 8>;
type Design = SmallVec<[u8; 64]>;
type Input = (Vec<Towel>, Vec<Design>);

#[aoc_generator(day19)]
fn parse(input: &str) -> Input {
    let input = input.as_bytes();
    let mut lines = input.split(|x| *x == b'\n');
    let towel_line = lines.next().unwrap();

    let mut towels = Vec::new();
    for towel in towel_line.split(|x| *x == b',') {
        let towel = towel.trim_ascii_start();
        let towel = ArrayVec::try_from(towel).unwrap();
        towels.push(towel);
    }

    let _empty_line = lines.next();
    let mut designs = Vec::new();
    for design in lines {
        designs.push(SmallVec::from_slice(design));
    }

    (towels, designs)
}

#[aoc(day19, part1)]
fn one<'i>((towels, designs): &'i Input) -> usize {
    let towels = towels.iter().sorted_by_key(|t| t.len());
    let mut neccessary_towels = Vec::with_capacity(towels.len());
    for towel in towels {
        if !(towels_can_form_design(&neccessary_towels, towel)) {
            neccessary_towels.insert(
                neccessary_towels.binary_search(towel).unwrap_err(),
                towel.clone(),
            );
        }
    }
    let towels = neccessary_towels;

    let mut memo = FxHashMap::default();

    designs
        .iter()
        .filter(|design| towels_can_form_design_memo(&towels, *design, &mut memo))
        .count()
}

fn towels_can_form_design_memo(
    towels: &[Towel],
    design: &[u8],
    memo: &mut FxHashMap<SmallVec<[u8; 64]>, bool>,
) -> bool {
    if design.is_empty() {
        true
    } else {
        if design.len() > 16 {
            for next_towel in initial_matching_towels(towels, design) {
                if towels_can_form_design_memo(towels, &design[next_towel.len()..], memo) {
                    return true;
                }
            }
            return false;
        }
        if let Some(res) = memo.get(design) {
            return *res;
        } else {
            let key = SmallVec::from_slice(design);
            for next_towel in initial_matching_towels(towels, design) {
                if towels_can_form_design_memo(towels, &design[next_towel.len()..], memo) {
                    memo.insert(key, true);
                    return true;
                }
            }
            memo.insert(key, false);
            return false;
        }
    }
}

fn towels_can_form_design(towels: &[Towel], design: &[u8]) -> bool {
    if design.is_empty() {
        true
    } else {
        for next_towel in initial_matching_towels(towels, design) {
            if towels_can_form_design(towels, &design[next_towel.len()..]) {
                return true;
            }
        }
        false
    }
}

fn initial_matching_towels<'a>(
    towels: &'a [Towel],
    design: &'a [u8],
) -> impl Iterator<Item = &'a Towel> {
    let base = towels.partition_point(|t| t[0] < design[0]);
    let size = towels[base..].partition_point(|t| t[0] == design[0]);
    towels[base..(base + size)]
        .iter()
        .filter(|t| design.starts_with(&t))
}

pub fn part1(puzzle: &str) -> usize {
    one(&parse(puzzle))
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(&include_str!("test.txt"));
        assert_eq!(res, 6);
    }
}
