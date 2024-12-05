use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::BTreeSet;

use crate::util::VecVec;

type Output = u32;
type PageNum = u32;
type Input = (BTreeSet<(PageNum, PageNum)>, VecVec<PageNum>);

#[aoc_generator(day5)]
fn parse(puzzle: &str) -> Input {
    let mut lines = puzzle.lines();
    let mut rules = BTreeSet::new();
    let mut pos = 0;
    for line in &mut lines {
        pos += line.len() + 1;
        if line.is_empty() {
            break;
        }
        let (left, right) = line.split_once('|').unwrap();
        rules.insert((
            left.parse::<PageNum>().unwrap(),
            right.parse::<PageNum>().unwrap(),
        ));
    }

    let pages_estimate = 1 + (puzzle.len() - pos) / 3;
    let mut updates = VecVec::with_capacity(pages_estimate);
    for line in lines {
        if line.is_empty() {
            break;
        }
        let pages = line
            .split(',')
            .map(str::parse::<PageNum>)
            .map(Result::unwrap);
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

fn is_legal(update: &[PageNum], rules: &BTreeSet<(PageNum, PageNum)>) -> bool {
    for i in 0..update.len() {
        let page = update[i];
        for later_page in update[i + 1..].iter() {
            if rules.contains(&(*later_page, page)) {
                return false;
            }
        }
    }
    // eprintln!("{update:?} is legal.");
    true
}

#[inline]
fn middle_page_num(update: &[PageNum]) -> Output {
    debug_assert!(update.len() % 2 == 1);
    update[(update.len()) / 2]
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
            update.sort_by(|a, b| {
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
