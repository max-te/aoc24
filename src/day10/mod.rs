use std::hash::Hash;

use crate::util::parse_digit;
use aoc_runner_derive::{aoc, aoc_generator};

type Output = usize;
type Input = Map;

type Coord = usize;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(usize);

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32((self.0 as u32) << 16 | self.1 as u32);
    }
}

struct Map {
    data: Vec<u8>,
    width: Coord,
    height: Coord,
}

impl Map {
    fn contains(&self, point: &Point) -> bool {
        (0..self.width).contains(&point.0) && (0..self.height).contains(&point.1)
    }

    fn get(&self, point: &Point) -> u8 {
        self.data[point.0]
    }

    fn neighbors(&self, point: &Point) -> impl Iterator<Item = Point> {
        &[Point(po)]
    }
}

#[aoc_generator(day10)]
fn parse(input: &[u8]) -> Input {
    let mut depths = Vec::with_capacity(input.len());
    let mut width = None;
    let mut height = 0;
    for ch in input {
        if *ch != b'\n' {
            depths.push(parse_digit(ch));
        } else {
            width.get_or_insert(depths.len());
            height += 1;
        }
    }
    if input.last() != Some(&b'\n') {
        height += 1;
    }
    Map {
        data: depths,
        width: width.unwrap(),
        height,
    }
}

#[aoc(day10, part1)]
fn one(map: &Input) -> Output {
    let trailheads = map
        .data
        .iter()
        .enumerate()
        .filter(|(_, d)| **d == 0)
        .map(|(i, _)| map.to_point(i));
    todo!()
}

#[aoc(day10, part2)]
fn two(map: &Input) -> Output {
    todo!()
}

pub fn part1(puzzle: &str) -> Output {
    one(&parse(puzzle.as_bytes()))
}

pub fn part2(puzzle: &str) -> Output {
    two(&parse(puzzle.as_bytes()))
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1_simple() {
        let res = one(&parse(include_str!("test_simple.txt")));
        assert_eq!(res, 1);
    }

    #[test]
    fn example1() {
        let res = one(&parse(include_str!("test.txt")));
        assert_eq!(res, 36);
    }

    // #[test]
    // fn example2() {
    //     let res = two(include_str!("test.txt"));
    //     assert_eq!(res, 2858);
    // }
}
