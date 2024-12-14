use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::{FxBuildHasher, FxHashSet};

use crate::util::parse_initial_digits;

type Num = i64;
#[derive(Debug, Clone, Copy)]
struct Robot {
    x: Num,
    y: Num,
    vel_x: Num,
    vel_y: Num,
}

type Input = Vec<Robot>;
type Output = usize;

#[aoc_generator(day14)]
fn parse(mut input: &[u8]) -> Input {
    let mut robots = Vec::with_capacity(input.len() / 10);
    loop {
        input = &input[const { "p=".len() }..];
        let (x, len) = parse_initial_digits(&input);
        input = &input[len + const { ",".len() }..];
        let (y, len) = parse_initial_digits(&input);
        input = &input[len + const { " v=".len() }..];
        let (vel_x, len) = parse_initial_digits(&input);
        input = &input[len + const { ",".len() }..];
        let (vel_y, len) = parse_initial_digits(&input);
        input = &input[len..];
        robots.push(Robot { x, y, vel_x, vel_y });
        if input.len() < 2 {
            break;
        } else {
            input = &input[1..];
        }
    }
    robots
}

fn one_inner(robots: &[Robot], width: Num, height: Num) -> Output {
    let middle_x = (width - 1) / 2;
    let middle_y = (height - 1) / 2;
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for robot in robots {
        let final_x = (robot.x + robot.vel_x * 100).rem_euclid(width);
        let final_y = (robot.y + robot.vel_y * 100).rem_euclid(height);

        if final_x < middle_x {
            if final_y < middle_y {
                q1 += 1;
            } else if final_y > middle_y {
                q2 += 1;
            }
        } else if final_x > middle_x {
            if final_y < middle_y {
                q3 += 1;
            } else if final_y > middle_y {
                q4 += 1;
            }
        }
    }
    q1 * q2 * q3 * q4
}

#[aoc(day14, part1)]
fn one(robots: &Input) -> Output {
    one_inner(robots, 101, 103)
}

fn has_frame_at(
    positions: &FxHashSet<(usize, usize)>,
    left_x: usize,
    top_y: usize,
    width: usize,
    height: usize,
) -> bool {
    for x in left_x..(left_x + width) {
        if !positions.contains(&(x, top_y)) {
            return false;
        }
    }
    for y in top_y..(top_y + height) {
        if !positions.contains(&(left_x, y)) {
            return false;
        }
    }
    true
}

#[aoc(day14, part2, naive)]
fn two(robots: &Input) -> Output {
    let width = 101;
    let height = 103;
    let mut robots = robots.to_vec();

    for step in 1..(101 * 103) {
        let mut positions =
            HashSet::with_capacity_and_hasher(robots.len(), FxBuildHasher::default());
        for robot in &mut robots {
            robot.x = (robot.x + robot.vel_x).rem_euclid(width);
            robot.y = (robot.y + robot.vel_y).rem_euclid(height);
            positions.insert((robot.x as usize, robot.y as usize));
        }
        for (x, y) in &positions {
            if has_frame_at(&positions, *x, *y, 31, 33) {
                return step;
            }
        }
    }
    panic!("Could not find christmas tree")
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
    fn example1() {
        let res = one_inner(&parse(include_bytes!("test.txt")), 11, 7);
        assert_eq!(res, 12);
    }
}
