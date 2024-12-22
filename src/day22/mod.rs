use aoc_runner_derive::aoc;
use itertools::Itertools;
use rayon::{
    iter::{ParallelBridge, ParallelIterator},
    slice::ParallelSlice,
};
use rustc_hash::FxHashMap;

const SECRET_MASK: u32 = 16777216 - 1;

#[inline]
fn next_secret(secret: u32) -> u32 {
    let secret = ((secret << 6) & SECRET_MASK) ^ secret;
    let secret = ((secret >> 5) & SECRET_MASK) ^ secret;
    let secret = ((secret << 11) & SECRET_MASK) ^ secret;
    secret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BananaTwister(u32);

impl Iterator for BananaTwister {
    type Item = u32;
    #[inline]
    fn next(&mut self) -> Option<u32> {
        let result = self.0;
        self.0 = next_secret(self.0);
        Some(result)
    }
}

#[inline]
fn prices(secret: u32) -> impl Iterator<Item = i8> {
    BananaTwister(secret).map(|secret| (secret % 10) as _)
}

#[aoc(day22, part1)]
pub fn part1(puzzle: &str) -> u64 {
    let mut res: u64 = 0;
    for mut secret in puzzle.lines().map(|l| l.parse::<u32>().unwrap()) {
        for _ in 0..2000 {
            secret = next_secret(secret);
        }
        res += secret as u64;
    }
    res
}

type SequenceValue = FxHashMap<(i8, i8, i8, i8), (usize, u32)>;

#[inline]
fn add_sequence_values(secret: u32, monkey_idx: usize, sequence_value: &mut SequenceValue) {
    let prices = prices(secret).take(2001);
    let differences = prices.tuple_windows().map(|(p1, p2)| (p2 - p1, p2));
    for ((d1, _), (d2, _), (d3, _), (d4, p4)) in differences.tuple_windows() {
        let changes = (d1, d2, d3, d4);
        let entry = sequence_value
            .entry(changes)
            .or_insert((monkey_idx, p4 as u32));
        if entry.0 != monkey_idx {
            *entry = (monkey_idx, entry.1 + p4 as u32);
        }
    }
}

#[aoc(day22, part2, hashmap)]
fn two(puzzle: &str) -> u32 {
    let mut sequence_value = FxHashMap::default();
    for (monkey_idx, secret) in puzzle
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .enumerate()
    {
        add_sequence_values(secret, monkey_idx, &mut sequence_value);
    }
    sequence_value.values().map(|v| v.1).max().unwrap()
}

const SEQUENCE_VALUE_TABLE_SIZE: usize = 19 * 19 * 19 * 19;

#[inline(always)]
fn sequence_to_index((d1, d2, d3, d4): (i8, i8, i8, i8)) -> usize {
    ((d1 + 9) as usize * 19 + (d2 + 9) as usize) * (19 * 19)
        + ((d3 + 9) as usize * 19 + (d4 + 9) as usize)
}

#[inline]
unsafe fn add_sequence_values_array(
    secret: u32,
    monkey_idx: u16,
    sequence_value: &mut [(u16, u16)],
) {
    let mut prices = prices(secret).take(2001);

    let p0 = prices.next().unwrap_unchecked();
    let p1 = prices.next().unwrap_unchecked();
    let p2 = prices.next().unwrap_unchecked();
    let p3 = prices.next().unwrap_unchecked();
    let mut d1 = p1 - p0;
    let mut d2 = p2 - p1;
    let mut d3 = p3 - p2;

    let mut previous_price = p3;
    while let Some(p) = prices.next() {
        let d4 = p - previous_price;
        let changes = (d1, d2, d3, d4);
        let entry = sequence_value.get_unchecked_mut(sequence_to_index(changes));
        if entry.0 != monkey_idx {
            *entry = (monkey_idx, entry.1 + p as u16);
        }
        previous_price = p;
        d1 = d2;
        d2 = d3;
        d3 = d4;
    }
}

#[aoc(day22, part2, array)]
fn two_array(puzzle: &str) -> u16 {
    let mut sequence_value = [(0, 0); SEQUENCE_VALUE_TABLE_SIZE];
    let mut monkey_idx = 1;
    for secret in puzzle.lines().map(|l| l.parse::<u32>().unwrap()) {
        unsafe { add_sequence_values_array(secret, monkey_idx, &mut sequence_value) };
        monkey_idx += 1;
    }
    sequence_value.iter().map(|v| v.1).max().unwrap()
}

#[aoc(day22, part2, array_rayon)]
fn two_array_rayon(puzzle: &str) -> u16 {
    let secrets = puzzle
        .lines()
        .par_bridge()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let sequence_value = secrets
        .par_chunks(128)
        .map(|chunk| {
            let mut sequence_value = vec![(0, 0); SEQUENCE_VALUE_TABLE_SIZE];
            let mut monkey_idx = 1;
            for secret in chunk {
                unsafe {
                    add_sequence_values_array(*secret, monkey_idx, sequence_value.as_mut_slice())
                };
                monkey_idx += 1;
            }
            sequence_value
        })
        .reduce_with(|mut a, b| {
            for i in 0..SEQUENCE_VALUE_TABLE_SIZE {
                a[i].1 += b[i].1;
            }
            a
        })
        .unwrap();

    sequence_value.iter().map(|v| v.1).max().unwrap()
}

#[aoc(day22, part2, hashmap_rayon)]
fn two_hashmap_rayon(puzzle: &str) -> u32 {
    let secrets = puzzle
        .lines()
        .par_bridge()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let sequence_value = secrets
        .par_chunks(128)
        .fold(
            || FxHashMap::default(),
            |mut sequence_value, chunk| {
                let mut monkey_idx = 1;
                for secret in chunk {
                    add_sequence_values(*secret, monkey_idx, &mut sequence_value);
                    monkey_idx += 1;
                }
                sequence_value
            },
        )
        .reduce_with(|mut a, b| {
            for (k, v) in b {
                let entry = a.entry(k).or_insert((0, 0));
                entry.1 += v.1;
            }
            a
        })
        .unwrap();

    sequence_value.iter().map(|v| v.1 .1).max().unwrap()
}

pub fn part2(puzzle: &str) -> u16 {
    two_array(puzzle)
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn test_part1_next_secret() {
        let mut secret = 123;
        secret = next_secret(secret);
        assert_eq!(secret, 15887950);
        secret = next_secret(secret);
        assert_eq!(secret, 16495136);
        secret = next_secret(secret);
        assert_eq!(secret, 527345);
        secret = next_secret(secret);
        assert_eq!(secret, 704524);
    }

    #[test]
    fn test_prices() {
        let prices = prices(123);
        assert_eq!(
            prices.take(10).collect::<Vec<_>>(),
            &[3, 0, 6, 5, 4, 4, 6, 4, 4, 2]
        );
    }

    #[test]
    fn example1() {
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, 37327623);
    }

    #[test]
    fn test_add_sequence_values() {
        let mut output = FxHashMap::default();
        add_sequence_values(1, 1, &mut output);
        assert_eq!(output.get(&(-2, 1, -1, 3)), Some(&(1, 7)));
        add_sequence_values(2, 2, &mut output);
        add_sequence_values(3, 3, &mut output);
        add_sequence_values(2024, 4, &mut output);
        assert_eq!(output.get(&(-2, 1, -1, 3)), Some(&(4, 23)));
    }

    #[test]
    fn test_periodicity() {
        let initial = 123;
        let mut secret = initial;
        for i in 0..2048 {
            secret = next_secret(secret);
            assert_ne!(secret, initial, "Periodicity detected at {}", i);
        }
    }
}
