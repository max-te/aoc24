use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nohash_hasher::BuildNoHashHasher;
use std::collections::HashMap;

use crate::util::{parse_2_digits, VecVec};

type Output = u32;
type PageNum = u8;
type Input = (Vec<PageNum>, VecVec<PageNum>);

// Assumption: the rules specify a total order.
// This is necessary to make part 2 well-defined.
// This allows us to just store the page order, and not the
// quadratic set of rules.

#[aoc_generator(day5)]
fn parse(puzzle: &str) -> Input {
    let mut lines = puzzle.lines();
    let mut rule_counts: HashMap<PageNum, i8, BuildNoHashHasher<PageNum>> =
        HashMap::with_capacity_and_hasher(100, BuildNoHashHasher::default());
    let mut pos = 0;
    for line in &mut lines {
        pos += line.len() + 1;
        if line.is_empty() {
            break;
        }
        let (left, right) = line.split_once('|').unwrap();
        let left = parse_2_digits(left.as_bytes());
        let right = parse_2_digits(right.as_bytes());
        rule_counts.entry(right).or_insert(0);
        *rule_counts.entry(left).or_insert(0) += 1;
    }

    let pages_in_order: Vec<PageNum> = rule_counts
        .keys()
        .copied()
        .sorted_unstable_by_key(|&page| -rule_counts[&page])
        .collect();

    let pages_estimate = 1 + (puzzle.len() - pos) / 3;
    let mut updates = VecVec::with_capacity(pages_estimate);
    for line in lines {
        if line.is_empty() {
            break;
        }
        let pages = line.split(',').map(str::as_bytes).map(parse_2_digits);
        updates.push_from(pages);
    }
    (pages_in_order, updates)
}

#[aoc(day5, part1)]
fn part_one((page_order, updates): &Input) -> Output {
    updates
        .iter()
        .filter(|update| is_legal(update, &page_order))
        .map(|update| middle_page_num(&update))
        .sum()
}

fn is_legal(update: &[PageNum], page_order: &Vec<PageNum>) -> bool {
    let mut iter_order = page_order.iter();

    // eprintln!("{update:?}, {page_order:?}");
    for page in update.iter() {
        match iter_order.find(|&p| p == page) {
            Some(_) => continue,
            None => return false,
        }
    }
    // eprintln!("{update:?} is legal.");
    true
}

#[inline]
fn middle_page_num(update: &[PageNum]) -> Output {
    debug_assert!(update.len() % 2 == 1);
    update[(update.len()) / 2] as Output
}

pub fn part1(puzzle: &str) -> Output {
    part_one(&parse(puzzle))
}

#[aoc(day5, part2)]
fn part_two((page_order, updates): &Input) -> Output {
    updates
        .iter()
        .filter(|update| !is_legal(update, &page_order))
        .map(|update| {
            page_order
                .iter()
                .filter(|&p| update.contains(p))
                .map(|&p| p)
                .collect_vec()
        })
        .map(|update| middle_page_num(&update))
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
        assert_eq!(res, 143);
    }

    #[test]
    fn example2() {
        let res = part2(include_str!("test.txt"));
        assert_eq!(res, 123);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn parses_example() {}
}
