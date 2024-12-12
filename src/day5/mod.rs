use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;
use std::cmp::Ordering;

use crate::util::{parse_2_digits, VecVec};

type Output = u32;
type PageNum = u8;
type Input = (FxHashSet<(PageNum, PageNum)>, VecVec<PageNum>);

#[aoc_generator(day5)]
fn parse(puzzle: &str) -> Input {
    let puzzle = puzzle.as_bytes();
    let mut rules = FxHashSet::default();
    let mut cursor = 0;
    while puzzle[cursor] != b'\n' {
        let line = &puzzle[cursor..];
        let (left, right) = (&line[0..=1], &line[3..=4]);
        rules.insert((parse_2_digits(left), parse_2_digits(right)));
        cursor += 6;
    }

    let pages_estimate = 1 + (puzzle.len() - cursor) / 3;
    let mut updates = VecVec::with_capacity(pages_estimate);
    for line in puzzle[cursor + 1..].split(|x| *x == b'\n') {
        if line.is_empty() {
            break;
        }
        let pages = line.chunks(3).map(|c| parse_2_digits(&c[..2]));
        updates.push_from(pages);
    }
    (rules, updates)
}

#[aoc(day5, part1)]
fn part_one((rules, updates): &Input) -> Output {
    updates
        .iter()
        .filter(|update| is_legal(update, &rules))
        .map(|update| middle_page_num(&update))
        .sum()
}

fn is_legal(update: &[PageNum], rules: &FxHashSet<(PageNum, PageNum)>) -> bool {
    for i in 0..update.len() - 1 {
        let page = update[i];
        // Assumption (info from part 2): rules specify a total or cyclic order, so we just need to check the next one
        let later_page = update[i + 1];
        if rules.contains(&(later_page, page)) {
            return false;
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

// Assumption: rules specify a total order.
#[aoc(day5, part2)]
fn part_two((rules, updates): &Input) -> Output {
    updates
        .iter()
        .filter(|update| !is_legal(update, &rules))
        .map(|update| {
            let mut update = update.to_vec();
            update.sort_unstable_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update
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
