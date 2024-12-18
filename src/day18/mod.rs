use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use std::hash::Hash;

type Output = usize;
type Input = Vec<Point>;

type Coord = u32;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(Coord, Coord);

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32((self.0 as u32) << 16 | self.1 as u32);
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| l.split_once(','))
        .map(|(a, b)| Point(a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

#[aoc(day18, part1)]
fn one(points: &[Point]) -> Output {
    one_inner::<70>(&points[..1024])
}

fn one_inner<const SIZE: Coord>(points: &[Point]) -> Output {
    let obstacles = FxHashSet::from_iter(points);
    let start = Point(0, 0);
    let res = pathfinding::directed::dijkstra::dijkstra(
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
    .unwrap();

    res.1
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = one_inner::<6>(&parse(include_str!("test.txt"))[..12]);
        assert_eq!(res, 22);
    }
}
