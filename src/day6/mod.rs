use aoc_runner_derive::{aoc, aoc_generator};
use indexmap::IndexSet;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

type NaiveInput = (HashSet<Point>, Guard, Coord, Coord);

#[aoc_generator(day6, naive)]
fn parse(puzzle: &str) -> NaiveInput {
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

#[aoc(day6, part1, naive)]
fn one_naive((obstacles, guard, width, height): &NaiveInput) -> Output {
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
    one_naive(&parse(puzzle))
}

#[derive(Debug)]
struct GuardPath {
    is_loop: bool,
    path: IndexSet<Guard>,
}

fn trace_path(obstacles: &HashSet<Point>, mut guard: Guard, width: Coord, height: Coord) -> GuardPath {
    let mut path = IndexSet::new();
    let mut is_loop = false;
    while guard.0.in_range(width, height) {
        if !path.insert(guard) {
            is_loop = true;
            break;
        }
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
    GuardPath {
        is_loop,
        path,
    }
}

impl GuardPath {
    fn _visits(&self, point: Point) -> bool {
        self.path.iter().find(|p| p.0.eq(&point)).is_some()
    }
}

fn _debug_draw(obstacles: &HashSet<Point>, guard: &Guard, width: Coord, height: Coord, path: &GuardPath, blocked: Option<Point>) {
    eprint!("  ");
    for x in 0..width {
        eprint!("{x}");
    }
    eprint!("\n");
    for y in 0..height {
        eprint!("{y:2}");
        for x in 0..width {
            let point = Point(x, y);
            if blocked == Some(point) {
                eprint!("O");
            } else if obstacles.contains(&point) {
                eprint!("#");
            } else if guard.0 == point {
                eprint!("G");
            } else if path._visits(point) {
                eprint!("o");
            } else {
                eprint!(".");                
            }
        }
        eprint!("\n");
    }
}

#[aoc(day6, part2, naive)]
fn two_naive((obstacles, guard, width, height): &NaiveInput) -> Output {
    let mut obstacles = obstacles.clone();
    let base_path = trace_path(&obstacles, guard.clone(), *width, *height);
    let mut blockable = HashSet::new();
    for next in base_path.path.iter().skip(1) {
        assert!(obstacles.insert(next.0));
        let path = trace_path(&obstacles, guard.clone(), *width, *height);
        if path.is_loop {
            blockable.insert(next.0);
        }
        assert!(obstacles.remove(&next.0));
    }
    blockable.len()
}

pub fn part2(puzzle: &str) -> Output {
    two_naive(&parse(puzzle))
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, 41);
    }

    #[test]
    fn example2() {
        let res = part2(include_str!("test.txt"));
        assert_eq!(res, 6);
    }
}
