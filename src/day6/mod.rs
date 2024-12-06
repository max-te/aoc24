use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Output = usize;
type Coord = i16;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(Coord, Coord);

impl Point {
    fn in_range(&self, width: Coord, height: Coord) -> bool {
        (0..width).contains(&self.0) && (0..height).contains(&self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    North,
    South,
    West,
    East,
}

impl Facing {
    fn step(self, point: Point) -> Point {
        match self {
            Facing::North => Point(point.0, point.1 - 1),
            Facing::South => Point(point.0, point.1 + 1),
            Facing::West => Point(point.0 - 1, point.1),
            Facing::East => Point(point.0 + 1, point.1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard(Point, Facing);

impl Guard {
    fn turn_right(&mut self) {
        self.1 = match self.1 {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        };
    }
}

type Input = (HashSet<Point>, Guard, Coord, Coord);

#[aoc_generator(day6)]
fn parse(puzzle: &str) -> Input {
    let puzzle = puzzle.as_bytes();
    let mut point = Point(0, 0);
    let mut obstacles = HashSet::new();
    let mut guard = None;
    let mut width = None;
    for ch in puzzle {
        match *ch {
            b'\n' => {
                width.get_or_insert(point.0);
                point.1 += 1;
                point.0 = 0;
                continue;
            }
            b'#' => {
                obstacles.insert(point.clone());
            }
            b'^' => {
                guard = Some(Guard(point.clone(), Facing::North));
            }
            b'>' => {
                guard = Some(Guard(point.clone(), Facing::East));
            }
            b'<' => {
                guard = Some(Guard(point.clone(), Facing::West));
            }
            b'v' => {
                guard = Some(Guard(point.clone(), Facing::South));
            }
            b'.' => {}
            _ => unreachable!(),
        }
        point.0 += 1;
    }

    (
        obstacles,
        guard.unwrap(),
        width.unwrap(),
        point.1,
    )
}

#[aoc(day6, part1)]
fn part_one((obstacles, guard, width, height): &Input) -> Output {
    let mut guard = *guard;
    let mut visited = HashSet::new();
    while guard.0.in_range(*width, *height) {
        visited.insert(guard.0.clone());
        loop {
            let step_pos = guard.1.step(guard.0);
            if obstacles.contains(&step_pos) {
                guard.turn_right();
            } else {
                guard.0 = step_pos;
                break;
            }
        }
    }
    visited.len()
}

pub fn part1(puzzle: &str) -> Output {
    part_one(&parse(puzzle))
}

#[aoc(day6, part2)]
fn part_two((obstacles, guard, width, height): &Input) -> Output {
    todo!()
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
        assert_eq!(res, 41);
    }

    // #[test]
    // fn example2() {
    //     let res = part2(include_str!("test.txt"));
    //     assert_eq!(res, todo!());
    // }
}
