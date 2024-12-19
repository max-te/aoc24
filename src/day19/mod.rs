use core::str;

use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use itertools::Itertools;
use rustc_hash::FxHashMap;

type Towel = ArrayVec<u8, 8>;
type Design = ArrayVec<u8, 64>;
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
        designs.push(ArrayVec::try_from(design).unwrap());
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
    memo: &mut FxHashMap<Design, bool>,
) -> bool {
    if design.is_empty() {
        true
    } else {
        if design.len() > 40 {
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
            let key = Design::try_from(design).unwrap();
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

#[derive(Debug, Clone)]
struct PrefixTowelsIterator<'t, 'm> {
    towels: &'t [Towel],
    needle: &'m [u8],
    position: usize,
}

impl<'t, 'm> Iterator for PrefixTowelsIterator<'t, 'm> {
    type Item = &'t Towel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.towels.is_empty() || self.needle.is_empty() {
            None
        } else {
            let (head, new_needle) = self.needle.split_at(1);
            let head = head[0];
            self.needle = new_needle;
            let base = self
                .towels
                .partition_point(|t| t.get(self.position).is_none_or(|x| *x < head));
            let size = self.towels[base..].partition_point(|t| t.get(self.position) == Some(&head));
            self.towels = &self.towels[base..(base + size)];
            self.position += 1;
            let front_towel = self.towels.get(0)?;
            if front_towel.len() == self.position {
                Some(&front_towel)
            } else {
                self.next()
            }
        }
    }
}

fn initial_matching_towels<'a>(
    towels: &'a [Towel],
    design: &'a [u8],
) -> impl Iterator<Item = &'a Towel> {
    PrefixTowelsIterator {
        towels,
        needle: &design,
        position: 0,
    }
}

#[aoc(day19, part2)]
fn two((towels, designs): &Input) -> usize {
    let towels = towels.iter().cloned().sorted().collect_vec();
    let mut memo = FxHashMap::default();

    designs
        .iter()
        .map(|design| count_towels_can_form_design_memo(&towels, design, &mut memo))
        .sum()
}

fn count_towels_can_form_design_memo(
    towels: &[Towel],
    design: &[u8],
    memo: &mut FxHashMap<Design, usize>,
) -> usize {
    if design.is_empty() {
        1
    } else {
        if let Some(res) = memo.get(design) {
            return *res;
        } else {
            let key = Design::try_from(design).unwrap();
            let mut count = 0;
            for next_towel in initial_matching_towels(towels, design) {
                count +=
                    count_towels_can_form_design_memo(towels, &design[next_towel.len()..], memo)
            }
            memo.insert(key, count);
            return count;
        }
    }
}

pub fn part1(puzzle: &str) -> usize {
    one(&parse(puzzle))
}

pub fn part2(puzzle: &str) -> usize {
    two(&parse(puzzle))
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(&include_str!("test.txt"));
        assert_eq!(res, 6);
    }
    #[test]
    fn example2() {
        let res = part2(&include_str!("test.txt"));
        assert_eq!(res, 16);
    }
}
