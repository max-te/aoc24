use aoc_runner_derive::aoc;
use itertools::Itertools;
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
fn prices(secret: u32) -> impl Iterator<Item = i32> {
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

#[inline]
fn get_sequence_values(secret: u32) -> FxHashMap<(i32, i32, i32, i32), u32> {
    let mut res = FxHashMap::default();
    let prices = prices(secret).take(2001);
    for (p0, p1, p2, p3, p4) in prices.tuple_windows() {
        let changes = (p1 - p0, p2 - p1, p3 - p2, p4 - p3);
        res.entry(changes).or_insert(p4 as u32);
    }
    res
}

#[inline]
fn add_hashmap<T: std::hash::Hash + Eq>(to: &mut FxHashMap<T, u32>, from: FxHashMap<T, u32>) {
    for (key, value) in from {
        *to.entry(key).or_insert(0) += value;
    }
}

#[aoc(day22, part2)]
pub fn part2(puzzle: &str) -> u32 {
    let mut sequence_value = FxHashMap::default();
    for secret in puzzle.lines().map(|l| l.parse::<u32>().unwrap()) {
        let buyer_values = get_sequence_values(secret);
        add_hashmap(&mut sequence_value, buyer_values);
    }

    *sequence_value.values().max().unwrap()
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
        let buyer_1 = get_sequence_values(1);
        assert_eq!(buyer_1.get(&(-2, 1, -1, 3)), Some(&7));
        add_hashmap(&mut output, buyer_1);
        add_hashmap(&mut output, get_sequence_values(2));
        add_hashmap(&mut output, get_sequence_values(3));
        add_hashmap(&mut output, get_sequence_values(2024));
        assert_eq!(output.get(&(-2, 1, -1, 3)), Some(&23));
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
