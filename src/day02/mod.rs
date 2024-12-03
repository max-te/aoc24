use crate::Solutions;
use lib_aoc::prelude::*;

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


impl Solution<DAY_02> for Solutions {
    type Input<'i> = VecVec<u64>;
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let mut input = VecVec::with_capacity(puzzle.len() / 16);
        for line in puzzle.lines() {
            input.push_from(line.split(' ').map(str::parse::<u64>).map(Result::unwrap));
        }
        input
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        input.iter().filter(|seq| sequence_is_safe(seq)).count() as u64
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        input.iter()
            .filter(|seq| sequence_is_safe(seq) || problem_dampable(&seq.to_vec()))
            .count() as u64
    }
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
        if step < 1 || step > 3 {
            // eprintln!("{seq:?} has a step of {step} at {prev}, {i}");
            return false;
        }
        prev = i;
    }
    // eprintln!("{seq:?} is safe");
    true
}

fn problem_dampable(seq: &Vec<u64>) -> bool {
    for i in 0..seq.len() {
        let mut dampened = seq.clone();
        dampened.remove(i);
        if sequence_is_safe(&dampened) {
            // eprintln!("{seq:?} is dampable at {i}");
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
