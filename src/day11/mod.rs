use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Output = usize;
type Num = usize;
type Input = Vec<Num>;

#[aoc_generator(day11)]
fn parse(input: &str) -> Input {
    let input = input.trim_end();
    input
        .split(" ")
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<Num>>()
}

fn _blink(stone: Num, times: u8) -> Output {
    let digits = stone.checked_ilog10().unwrap_or_default() + 1;
    if times == 0 {
        1
    } else if stone == 0 {
        _blink(1, times - 1)
    } else if digits % 2 == 0 {
        let split = (10 as Num).pow(digits / 2);
        let first_half = stone / split;
        let second_half = stone % split;
        _blink(first_half, times - 1) + _blink(second_half, times - 1)
    } else {
        _blink(stone * 2024, times - 1)
    }
}

#[aoc(day11, part1)]
fn one(stones: &Input) -> Output {
    let mut answer = 0;
    let mut memo = HashMap::new();
    for stone in stones {
        answer += blink_memo(*stone, 25, &mut memo);
    }
    answer
}

fn blink_memo(stone: Num, times: u8, memory: &mut HashMap<(Num, u8), Output>) -> Output {
    let key = (stone, times);
    if let Some(res) = memory.get(&key) {
        return *res;
    }

    let digits = stone.checked_ilog10().unwrap_or_default() + 1;
    let res = if times == 0 {
        1
    } else if stone == 0 {
        blink_memo(1, times - 1, memory)
    } else if digits % 2 == 0 {
        let split = (10 as Num).pow(digits / 2);
        let first_half = stone / split;
        let second_half = stone % split;
        blink_memo(first_half, times - 1, memory) + blink_memo(second_half, times - 1, memory)
    } else {
        blink_memo(stone * 2024, times - 1, memory)
    };
    memory.insert(key, res);
    res
}

#[aoc(day11, part2)]
fn two(stones: &Input) -> Output {
    let mut answer = 0;
    let mut memo = HashMap::new();
    for stone in stones {
        answer += blink_memo(*stone, 75, &mut memo);
    }
    answer
}

pub fn part1(puzzle: &str) -> Output {
    one(&parse(puzzle))
}

pub fn part2(puzzle: &str) -> Output {
    two(&parse(puzzle))
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = one(&parse(include_str!("test.txt")));
        assert_eq!(res, 55312);
    }

    // #[test]
    // fn example2() {
    //     let res = two(&parse(include_str!("test.txt")));
    //     assert_eq!(res, 81);
    // }
}
