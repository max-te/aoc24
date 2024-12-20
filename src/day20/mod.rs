use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::{matrix::Matrix, prelude::bfs};

use crate::util::first_line_length;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tile {
    #[default]
    Wall,
    Track(usize),
}

type Point = (usize, usize);
type Input = (Matrix<Tile>, Point, Point);

#[aoc_generator(day20)]
fn parse(input: &str) -> Input {
    let input = input.as_bytes().trim_ascii_end();
    let mut start = None;
    let mut finish = None;
    let mut map = Matrix::<Tile>::new_square(first_line_length(input), Tile::Wall);
    for (x, row) in input.split(|ch| *ch == b'\n').enumerate() {
        for (y, &ch) in row.iter().enumerate() {
            match ch {
                b'.' => map[(x, y)] = Tile::Track(usize::MAX),
                b'S' => {
                    start = Some((x, y));
                    map[(x, y)] = Tile::Track(usize::MAX)
                }
                b'E' => {
                    finish = Some((x, y));
                    map[(x, y)] = Tile::Track(usize::MAX)
                }
                b'#' => (),
                _ => unreachable!(),
            }
        }
    }

    (map, start.unwrap(), finish.unwrap())
}

const SHORTCUT_DIRECTIONS: [(isize, isize); 8] = [
    (-2, 0),
    (-1, -1),
    (0, -2),
    (1, -1),
    (2, 0),
    (1, 1),
    (0, 2),
    (-1, 1),
];

fn one_inner((map, start, finish): &Input, min_save: usize) -> usize {
    let mut map = map.clone();
    let track = bfs(
        start,
        |t| {
            map.neighbours(*t, false)
                .filter(|t| matches!(map[*t], Tile::Track(_)))
        },
        |t| t == finish,
    )
    .unwrap();
    let mut shortcuts = 0;

    for (time, pos) in track.iter().enumerate() {
        map[*pos] = Tile::Track(time);
        for target in SHORTCUT_DIRECTIONS
            .iter()
            .filter_map(|dir| map.move_in_direction(*pos, *dir))
        {
            if let Tile::Track(target_time) = map[target] {
                if time.saturating_sub(target_time) >= 2 + min_save {
                    #[cfg(test)]
                    eprintln!(
                        "Shortcut to {pos:?} from {target:?} to time {time} from {target_time}"
                    );
                    shortcuts += 1;
                };
            }
        }
    }
    shortcuts
}

fn one_inner_dual((map, start, finish): &Input, min_save: usize) -> usize {
    let track = bfs(
        start,
        |t| {
            map.neighbours(*t, false)
                .filter(|t| matches!(map[*t], Tile::Track(_)))
        },
        |t| t == finish,
    )
    .unwrap();
    let mut shortcuts = 0;
    let min_save_with_cost = min_save + 2;

    for (t_from, pos_from) in track[..(track.len() - min_save_with_cost)]
        .iter()
        .enumerate()
    {
        for pos_to in &track[t_from + min_save_with_cost..] {
            if manhattan(*pos_from, *pos_to) == 2 {
                shortcuts += 1;
            }
        }
    }
    shortcuts
}

#[inline]
fn manhattan(pos_from: (usize, usize), pos_to: (usize, usize)) -> usize {
    pos_from.0.abs_diff(pos_to.0) + pos_from.1.abs_diff(pos_to.1)
}

#[aoc(day20, part1, naive)]
fn one(input: &Input) -> usize {
    one_inner(input, 100)
}

#[aoc(day20, part1, dual)]
fn one_dual(input: &Input) -> usize {
    one_inner_dual(input, 100)
}

const fn manhattan_diamond<const RADIUS: usize, const LEN: usize>() -> [(isize, isize); LEN] {
    assert!(LEN == 4 * RADIUS);
    let mut i: usize = 0;
    let mut out = [(0, 0); LEN];
    while i < RADIUS {
        out[i] = (i as isize, (RADIUS - i) as isize);
        out[i + RADIUS] = ((RADIUS - i) as isize, -(i as isize));
        out[i + 2 * RADIUS] = (-(i as isize), -((RADIUS - i) as isize));
        out[i + 3 * RADIUS] = (-((RADIUS - i) as isize), i as isize);
        i += 1;
    }
    out
}

const fn const_concat<const LEN1: usize, const LEN2: usize, const LEN_SUM: usize>(
    a: [(isize, isize); LEN1],
    b: [(isize, isize); LEN2],
) -> [(isize, isize); LEN_SUM] {
    assert!(LEN1 + LEN2 == LEN_SUM);
    let mut out = [(0, 0); LEN_SUM];
    let mut i = 0;
    while i < LEN1 {
        out[i] = a[i];
        i += 1;
    }
    while i < LEN_SUM {
        out[i] = b[i - LEN1];
        i += 1;
    }
    out
}

const fn const_bubble_sort<const LEN: usize>(a: [(isize, isize); LEN]) -> [(isize, isize); LEN] {
    let mut out = a;
    let mut i = 0;
    while i < LEN {
        let mut j = i + 1;
        while j < LEN {
            if out[j].0 < out[i].0 || (out[j].0 == out[i].0 && out[j].1 < out[i].1) {
                let tmp = out[i];
                out[i] = out[j];
                out[j] = tmp;
            }
            j += 1;
        }
        i += 1;
    }
    out
}

const fn precompute_lengths<const LEN: usize>(
    a: [(isize, isize); LEN],
) -> [((isize, isize), usize); LEN] {
    let mut out = [((0, 0), 0); LEN];
    let mut i = 0;
    while i < LEN {
        out[i] = (a[i], (a[i].0.abs() + a[i].1.abs()) as usize);
        i += 1;
    }
    out
}

macro_rules! make_manhattan_diamond {
    ($radius:expr) => {
        const_bubble_sort(manhattan_diamond::<$radius, { 4 * $radius }>())
    };
}

macro_rules! sum_varargs {
    ($summand:expr) => {
        $summand
    };
    ($summand:expr, $($rest:expr),+) => {
        ($summand + sum_varargs!($($rest),+))
    }
}

macro_rules! make_manhattan_diamonds {
    ($radius:expr) => {
        make_manhattan_diamond!($radius)
    };
    ($radius:expr, $($rest:expr),+) => {
        const_concat::<{ 4 * $radius}, { 4 * sum_varargs!($($rest),+) }, { 4 * sum_varargs!($radius, $($rest),+) }>(make_manhattan_diamond!($radius), make_manhattan_diamonds!($($rest),+))
    }
}

const LONG_SHORTCUTS: &[((isize, isize), usize)] = const {
    &precompute_lengths(const_bubble_sort(make_manhattan_diamonds!(
        2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    )))
};

fn two_inner((map, start, finish): &Input, min_save: usize) -> usize {
    let mut map = map.clone();
    let track = bfs(
        start,
        |t| {
            map.neighbours(*t, false)
                .filter(|t| matches!(map[*t], Tile::Track(_)))
        },
        |t| t == finish,
    )
    .unwrap();
    let mut shortcuts = 0;

    for (time, pos) in track.iter().enumerate() {
        map[*pos] = Tile::Track(time);
        for (target, cheat_duration) in LONG_SHORTCUTS
            .iter()
            .filter_map(|(dir, len)| map.move_in_direction(*pos, *dir).map(|t| (t, *len)))
        {
            if let Tile::Track(target_time) = map[target] {
                if time.saturating_sub(target_time) >= cheat_duration as usize + min_save {
                    #[cfg(test)]
                    eprintln!(
                    "{cheat_duration}-Shortcut to {pos:?} from {target:?} to time {time} from {target_time}"
                );
                    shortcuts += 1;
                };
            }
        }
    }
    shortcuts
}

#[aoc(day20, part2, naive)]
fn two(input: &Input) -> usize {
    two_inner(input, 100)
}

pub fn part1(puzzle: &str) -> usize {
    one(&parse(puzzle))
}

pub fn part2(puzzle: &str) -> usize {
    two(&parse(puzzle))
}

#[cfg(test)]
mod examples {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn example1() {
        let res = one_inner(&parse(include_str!("test.txt")), 20);
        assert_eq!(res, 5);
    }

    #[test]
    fn example1_dual() {
        let res = one_inner_dual(&parse(include_str!("test.txt")), 20);
        assert_eq!(res, 5);
    }

    #[test]
    fn example2() {
        let res = two_inner(&parse(include_str!("test.txt")), 50);
        assert_eq!(
            res,
            [32, 31, 29, 39, 25, 23, 20, 19, 12, 14, 12, 22, 4, 3]
                .iter()
                .sum()
        );
    }

    #[test]
    fn manhattan_test() {
        assert_eq!(
            manhattan_diamond::<2, 8>().iter().sorted().collect_vec(),
            SHORTCUT_DIRECTIONS.iter().sorted().collect_vec()
        );
    }
}
