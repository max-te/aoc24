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
fn two_naive(robots: &Input) -> Output {
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

const WIDTH: Num = 101;
const HEIGHT: Num = 103;

#[aoc(day14, part2, chinese_remainder_theorem)]
fn two(robots: &Input) -> Output {
    let mut robots = robots.to_vec();

    let mut x_off = None;
    let mut y_off = None;
    let mut step = 0;

    while x_off.is_none() || y_off.is_none() {
        let mut col_counts = [0; WIDTH as usize];
        let mut row_counts = [0; HEIGHT as usize];
        for robot in &mut robots {
            robot.x = (robot.x + robot.vel_x).rem_euclid(WIDTH);
            robot.y = (robot.y + robot.vel_y).rem_euclid(HEIGHT);
            let x = robot.x as usize;
            col_counts[x] += 1;
            let y = robot.y as usize;
            row_counts[y] += 1;
        }
        step += 1;

        if x_off.is_none() {
            for x in 0..((WIDTH - 30) as usize) {
                if col_counts[x] >= 33 && col_counts[x + 30] >= 33 {
                    x_off = Some(step);
                    break;
                }
            }
        }
        if y_off.is_none() {
            for y in 0..((HEIGHT - 32) as usize) {
                if row_counts[y] >= 31 && row_counts[y + 32] >= 31 {
                    y_off = Some(step);
                    break;
                }
            }
        }
    }
    let a = x_off.unwrap();
    let b = y_off.unwrap();

    solve_chinese_remainder::<WIDTH, HEIGHT>(a, b) as usize
}

fn solve_chinese_remainder<const N: Num, const M: Num>(a: Num, b: Num) -> Num {
    let (y, _z, d) = const { extended_euclidian(N, M) };

    let ans = a - y * N * (a - b) / d;
    let ans = ans.rem_euclid(N * M / d);
    debug_assert_eq!(ans % N, a);
    debug_assert_eq!(ans % M, b);
    ans
}

/// returns (x, y, d) such that ax + by = g = gcd(a, b)
const fn extended_euclidian(a: Num, b: Num) -> (Num, Num, Num) {
    let mut s = 0;
    let mut old_s = 1;
    let mut r = b;
    let mut old_r = a;
    while r != 0 {
        let q = old_r.div_euclid(r);
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    let bezout_t = if b != 0 {
        (old_r - old_s * a).div_euclid(b)
    } else {
        0
    };
    debug_assert!(old_r == old_s * a + bezout_t * b);

    (old_s, bezout_t, old_r)
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
