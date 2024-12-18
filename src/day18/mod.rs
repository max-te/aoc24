use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::{directed::dijkstra::dijkstra, prelude::astar};
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use std::hash::Hash;

use crate::util::parse_initial_digits;

type Input = Vec<Point>;

type Coord = u16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(Coord, Coord);

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32((self.0 as u32) << 16 | self.1 as u32);
    }
}

#[inline]
#[aoc_generator(day18)]
fn parse(input: &str) -> Input {
    let mut input = input.as_bytes();
    let mut points = Vec::<Point>::with_capacity(input.len() / 4);
    while !input.is_empty() {
        let (x, num_len) = parse_initial_digits(input);
        input = &input[num_len + 1..];
        let (y, num_len) = parse_initial_digits(input);
        points.push(Point(x as u16, y as u16));
        input = &input[num_len..];
        if !input.is_empty() {
            input = &input[1..];
        }
    }
    points
}

#[inline]
#[aoc(day18, part1)]
fn one(points: &[Point]) -> Coord {
    one_inner::<70>(&points[..1024])
}

#[inline]
fn one_inner<const SIZE: Coord>(points: &[Point]) -> Coord {
    find_path_across::<SIZE>(points).unwrap().1
}

#[inline]
#[aoc(day18, part1, astar)]
fn one_astar(points: &[Point]) -> Coord {
    find_path_across_astar::<70>(&points[..1024]).unwrap().1
}

#[inline]
fn find_path_across<const SIZE: Coord>(points: &[Point]) -> Option<(Vec<Point>, Coord)> {
    let obstacles = FxHashSet::from_iter(points);
    let start = Point(0, 0);
    dijkstra(
        &start,
        #[inline]
        |node: &Point| {
            let mut neigh = SmallVec::<[_; 4]>::new();
            if node.1 < SIZE {
                let south = Point(node.0, node.1 + 1);
                if !obstacles.contains(&south) {
                    neigh.push((south, 1));
                }
            }
            if node.0 < SIZE {
                let east = Point(node.0 + 1, node.1);
                if !obstacles.contains(&east) {
                    neigh.push((east, 1));
                }
            }
            if node.1 > 0 {
                let north = Point(node.0, node.1 - 1);
                if !obstacles.contains(&north) {
                    neigh.push((north, 1));
                }
            }

            if node.0 > 0 {
                let west = Point(node.0 - 1, node.1);
                if !obstacles.contains(&west) {
                    neigh.push((west, 1));
                }
            }
            neigh
        },
        #[inline(always)]
        |node| node.0 == SIZE && node.1 == SIZE,
    )
}

#[inline]
fn find_path_across_astar<const SIZE: Coord>(points: &[Point]) -> Option<(Vec<Point>, Coord)> {
    let obstacles = FxHashSet::from_iter(points);
    let start = Point(0, 0);
    astar(
        &start,
        #[inline]
        |node: &Point| {
            let mut neigh = SmallVec::<[_; 4]>::new();
            if node.1 < SIZE {
                let south = Point(node.0, node.1 + 1);
                if !obstacles.contains(&south) {
                    neigh.push((south, 1));
                }
            }
            if node.0 < SIZE {
                let east = Point(node.0 + 1, node.1);
                if !obstacles.contains(&east) {
                    neigh.push((east, 1));
                }
            }
            if node.1 > 0 {
                let north = Point(node.0, node.1 - 1);
                if !obstacles.contains(&north) {
                    neigh.push((north, 1));
                }
            }

            if node.0 > 0 {
                let west = Point(node.0 - 1, node.1);
                if !obstacles.contains(&west) {
                    neigh.push((west, 1));
                }
            }
            neigh
        },
        #[inline]
        |node| 2 * SIZE - node.0 - node.1,
        #[inline(always)]
        |node| node.0 == SIZE && node.1 == SIZE,
    )
}

#[inline]
fn find_path_across_map<const SIZE: Coord>(
    obstacles: &FxHashMap<&Point, usize>,
    time: usize,
) -> Option<(Vec<Point>, Coord)> {
    let start = Point(0, 0);
    dijkstra(
        &start,
        #[inline]
        |node: &Point| {
            let mut neigh = SmallVec::<[_; 4]>::new();
            if node.1 < SIZE {
                let south = Point(node.0, node.1 + 1);
                if !obstacles.get(&south).is_some_and(|t| *t < time) {
                    neigh.push((south, 1));
                }
            }
            if node.0 < SIZE {
                let east = Point(node.0 + 1, node.1);
                if !obstacles.get(&east).is_some_and(|t| *t < time) {
                    neigh.push((east, 1));
                }
            }
            if node.1 > 0 {
                let north = Point(node.0, node.1 - 1);
                if !obstacles.get(&north).is_some_and(|t| *t < time) {
                    neigh.push((north, 1));
                }
            }

            if node.0 > 0 {
                let west = Point(node.0 - 1, node.1);
                if !obstacles.get(&west).is_some_and(|t| *t < time) {
                    neigh.push((west, 1));
                }
            }
            neigh
        },
        #[inline(always)]
        |node| node.0 == SIZE && node.1 == SIZE,
    )
}

#[inline]
#[aoc(day18, part2, blockade_dijkstra)]
fn two(points: &[Point]) -> String {
    let solution = two_inner::<70>(points);
    format!("{},{}", solution.0, solution.1)
}

#[inline]
fn two_inner<const SIZE: Coord>(points: &[Point]) -> Point {
    let drop_time = FxHashMap::from_iter(points.iter().enumerate().map(|(i, p)| (p, i)));

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Node {
        Spacetime(usize, Point),
        Waiting(usize),
    }
    let start = Node::Waiting(0);
    #[cfg(test)]
    dbg!(&points);

    let res = dijkstra(
        &start,
        |node| {
            let mut neigh = SmallVec::<[(Node, usize); 10]>::new();
            match node {
                Node::Spacetime(time, tile) => {
                    for (dx, dy) in [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                        (1, 0),
                        (1, -1),
                        (0, -1),
                    ] {
                        let adj_tile = Point(
                            tile.0.wrapping_add_signed(dx),
                            tile.1.wrapping_add_signed(dy),
                        );
                        if drop_time.get(&adj_tile).is_some_and(|t| t <= time) {
                            neigh.push((Node::Spacetime(*time, adj_tile), 0));
                            #[cfg(test)]
                            println!("From {tile:?} @ {time} to {adj_tile:?}.");
                        }
                    }
                    if *time < points.len() {
                        neigh.push((Node::Spacetime(*time + 1, *tile), 1));
                    }
                }
                Node::Waiting(time) => {
                    if *time < points.len() {
                        let new_point = points[*time];
                        if new_point.0 == 0 || new_point.1 == SIZE {
                            neigh.push((Node::Spacetime(*time, new_point), 0));
                        }
                        neigh.push((Node::Waiting(time + 1), 1));
                    }
                }
            }
            neigh
        },
        |node| match node {
            Node::Spacetime(_, point) => point.0 == SIZE || point.1 == 0,
            Node::Waiting(_) => false,
        },
    )
    .unwrap();
    #[cfg(debug_assertions)]
    dbg!(&res);

    points[res.1].clone()
}

#[inline]
#[aoc(day18, part2, blockade_astar)]
fn two_astar(points: &[Point]) -> String {
    let solution = two_inner_astar::<70>(points);
    format!("{},{}", solution.0, solution.1)
}

#[inline]
fn two_inner_astar<const SIZE: Coord>(points: &[Point]) -> Point {
    let drop_time = FxHashMap::from_iter(points.iter().enumerate().map(|(i, p)| (p, i)));

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Node {
        Spacetime(usize, Point),
        Waiting(usize),
    }
    let start = Node::Waiting(0);
    #[cfg(test)]
    dbg!(&points);

    let res = astar(
        &start,
        #[inline]
        |node| {
            let mut neigh = SmallVec::<[(Node, usize); 10]>::new();
            match node {
                Node::Spacetime(time, tile) => {
                    for (dx, dy) in [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                        (1, 0),
                        (1, -1),
                        (0, -1),
                    ] {
                        let adj_tile = Point(
                            tile.0.wrapping_add_signed(dx),
                            tile.1.wrapping_add_signed(dy),
                        );
                        if drop_time.get(&adj_tile).is_some_and(|t| t <= time) {
                            neigh.push((Node::Spacetime(*time, adj_tile), 1));
                            #[cfg(test)]
                            println!("From {tile:?} @ {time} to {adj_tile:?}.");
                        }
                    }
                    if *time < points.len() {
                        neigh.push((Node::Spacetime(*time + 1, *tile), 1 << 16));
                    }
                }
                Node::Waiting(time) => {
                    if *time < points.len() {
                        let new_point = points[*time];
                        if new_point.0 == 0 || new_point.1 == SIZE {
                            neigh.push((Node::Spacetime(*time, new_point), 1));
                        }
                        neigh.push((Node::Waiting(time + 1), 1 << 16));
                    }
                }
            }
            neigh
        },
        #[inline]
        |node| {
            match node {
                Node::Spacetime(_, point) => (SIZE - point.0).max(point.1) as usize,
                Node::Waiting(_) => 2, // = min length of a diagonal wall
            }
        },
        #[inline]
        |node| match node {
            Node::Spacetime(_, point) => point.0 == SIZE || point.1 == 0,
            Node::Waiting(_) => false,
        },
    )
    .unwrap();
    #[cfg(debug_assertions)]
    dbg!(&res);

    points[res.1 >> 16].clone()
}

#[inline]
#[aoc(day18, part2, binary_search)]
fn two_binary_search(points: &[Point]) -> String {
    let indexed = points.iter().enumerate().collect::<Vec<_>>();
    let p = indexed.partition_point(|(i, _)| find_path_across::<70>(&points[..*i]).is_some());
    let solution = points[p - 1];
    format!("{},{}", solution.0, solution.1)
}

#[inline]
#[aoc(day18, part2, binary_search_map)]
fn two_binary_search_map(points: &[Point]) -> String {
    let drop_time = FxHashMap::from_iter(points.iter().enumerate().map(|(i, p)| (p, i)));

    let mut base = 1024; // From part 1
    let mut size = points.len() - base;

    while size > 1 {
        let half = size / 2;
        let mid = base + half;

        let can_cross = find_path_across_map::<70>(&drop_time, mid).is_some();
        base = if can_cross { mid } else { base };

        size -= half;
    }

    let solution = points[base];
    format!("{},{}", solution.0, solution.1)
}

#[inline]
#[aoc(day18, part2, binary_search_astar)]
fn two_binary_search_astar(points: &[Point]) -> String {
    let indexed = points.iter().enumerate().collect::<Vec<_>>();
    let p = indexed.partition_point(|(i, _)| find_path_across_astar::<70>(&points[..*i]).is_some());
    let solution = points[p - 1];
    format!("{},{}", solution.0, solution.1)
}

pub fn part1(puzzle: &str) -> Coord {
    one(&parse(puzzle))
}

pub fn part2(puzzle: &str) -> String {
    two_binary_search_map(&parse(puzzle))
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = one_inner::<6>(&parse(include_str!("test.txt"))[..12]);
        assert_eq!(res, 22);
    }

    #[test]
    fn example2() {
        let res = two_inner::<6>(&parse(include_str!("test.txt")));
        assert_eq!(res, Point(6, 1));
    }

    #[test]
    fn example2_astar() {
        let res = two_inner_astar::<6>(&parse(include_str!("test.txt")));
        assert_eq!(res, Point(6, 1));
    }
}
