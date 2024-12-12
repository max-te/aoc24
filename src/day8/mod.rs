use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

type Output = usize;
type Coord = i16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(Coord, Coord);

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32((self.0 as u32) << 16 | self.1 as u32);
    }
}

impl Point {
    fn delta(&self, other: &Point) -> Delta {
        Delta(self.0 - other.0, self.1 - other.1)
    }

    fn plus(&self, delta: &Delta) -> Point {
        Point(self.0 + delta.0, self.1 + delta.1)
    }

    fn minus(&self, delta: &Delta) -> Point {
        Point(self.0 - delta.0, self.1 - delta.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Delta(Coord, Coord);

#[derive(Debug, Clone)]
struct Map {
    width: Coord,
    height: Coord,
    towers: FxHashMap<u8, Vec<Point>>,
}

impl Map {
    #[inline]
    fn covers(&self, point: &Point) -> bool {
        (0..self.width).contains(&point.0) && (0..self.height).contains(&point.1)
    }

    fn size(&self) -> Coord {
        self.width * self.height
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_buffer: Vec<u8> =
            Vec::with_capacity((self.width as usize + 1) * self.height as usize);
        for _y in 0..self.height {
            for _x in 0..self.width {
                map_buffer.push(b'.');
            }
            map_buffer.push(b'\n');
        }

        for (ch, points) in &self.towers {
            for point in points {
                if self.covers(point) {
                    map_buffer[point.1 as usize * (self.width as usize + 1) + point.0 as usize] =
                        *ch;
                }
            }
        }
        f.write_str(std::str::from_utf8(&map_buffer).unwrap())?;
        Ok(())
    }
}

type Input = Map;

#[aoc_generator(day8)]
fn parse(puzzle: &str) -> Input {
    let puzzle = puzzle.as_bytes();
    let mut point = Point(0, 0);
    let mut towers: FxHashMap<u8, Vec<Point>> =
        HashMap::with_capacity_and_hasher(128, FxBuildHasher::default());
    let mut width = None;
    for ch in puzzle {
        match *ch {
            b'\n' => {
                width.get_or_insert(point.0);
                point.1 += 1;
                point.0 = 0;
                continue;
            }
            b'.' => {}
            ch => towers
                .entry(ch)
                .or_insert_with(|| Vec::with_capacity(10))
                .push(point),
        }
        point.0 += 1;
    }
    if puzzle.last() != Some(&b'\n') {
        point.1 += 1;
    }
    Map {
        width: width.unwrap(),
        height: point.1,
        towers,
    }
}

#[aoc(day8, part1)]
fn one(map: &Input) -> Output {
    let mut antinodes =
        HashSet::with_capacity_and_hasher((map.size() / 10) as usize, FxBuildHasher::default());
    for points in map.towers.values() {
        for (i, first) in points.iter().enumerate() {
            for second in &points[i + 1..] {
                let delta = first.delta(second);
                let antinode = first.plus(&delta);
                debug_assert_ne!(&antinode, second);
                if map.covers(&antinode) {
                    antinodes.insert(antinode);
                }
                let antinode = second.minus(&delta);
                debug_assert_ne!(&antinode, first);
                if map.covers(&antinode) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}

#[aoc(day8, part2)]
fn two(map: &Input) -> Output {
    let mut antinodes =
        HashSet::with_capacity_and_hasher((map.size() / 2) as usize, FxBuildHasher::default());
    for points in map.towers.values() {
        for (i, first) in points.iter().enumerate() {
            for second in &points[i + 1..] {
                let delta = first.delta(second);
                let mut antinode = *first;
                while map.covers(&antinode) {
                    antinodes.insert(antinode);
                    antinode = antinode.plus(&delta)
                }
                antinode = *second;
                while map.covers(&antinode) {
                    antinodes.insert(antinode);
                    antinode = antinode.minus(&delta)
                }
            }
        }
    }
    antinodes.len()
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
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, 14);
    }

    #[test]
    fn example2() {
        let res = part2(include_str!("test.txt"));
        assert_eq!(res, 34);
    }

    #[test]
    fn example2_simple() {
        let res = part2(include_str!("test_simple.txt"));
        assert_eq!(res, 9);
    }
}
