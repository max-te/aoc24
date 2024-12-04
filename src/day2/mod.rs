use aoc_runner_derive::{aoc, aoc_generator};

pub struct VecVec<T> {
    lengths: Vec<usize>,
    data: Vec<T>,
}

impl VecVec<u64> {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            lengths: Vec::with_capacity(capacity),
            data: Vec::with_capacity(capacity),
        }
    }

    fn push_from<I: Iterator<Item = u64>>(&mut self, values: I) {
        let previous_data_length = self.data.len();
        self.data.extend(values);
        self.lengths.push(self.data.len() - previous_data_length);
    }

    fn iter(&self) -> VecVecIter {
        VecVecIter {
            data: self,
            lengths_index: 0,
            data_index: 0,
        }
    }
}

struct VecVecIter<'this> {
    data: &'this VecVec<u64>,
    lengths_index: usize,
    data_index: usize,
}

impl<'this> Iterator for VecVecIter<'this> {
    type Item = &'this [u64];

    fn next(&mut self) -> Option<Self::Item> {
        if self.lengths_index < self.data.lengths.len() {
            let length = self.data.lengths[self.lengths_index];
            self.lengths_index += 1;
            let start = self.data_index;
            self.data_index += length;
            Some(&self.data.data[start..self.data_index])
        } else {
            None
        }
    }
}

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
