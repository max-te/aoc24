use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::{
    directed::{bfs::bfs, dijkstra::dijkstra},
    prelude::astar,
};
use petgraph::{csr::IndexType, unionfind::UnionFind};
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use std::{cmp::Ordering, hash::Hash};

use crate::util::parse_initial_digits;

type Input = Vec<Point>;

type Coord = u16;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
fn one(points: &[Point]) -> usize {
    one_inner::<70>(&points[..1024])
}

#[inline]
fn one_inner<const SIZE: Coord>(points: &[Point]) -> usize {
    find_path_across::<SIZE>(points).unwrap().len() - 1
}

#[inline]
#[aoc(day18, part1, astar)]
fn one_astar(points: &[Point]) -> Coord {
    find_path_across_astar::<70>(&points[..1024]).unwrap().1
}

#[inline]
fn find_path_across<const SIZE: Coord>(points: &[Point]) -> Option<Vec<Point>> {
    let obstacles = FxHashSet::from_iter(points);
    let start = Point(0, 0);
    bfs(
        &start,
        #[inline]
        |node: &Point| {
            let mut neigh = SmallVec::<[_; 4]>::new();
            if node.1 < SIZE {
                let south = Point(node.0, node.1 + 1);
                if !obstacles.contains(&south) {
                    neigh.push(south);
                }
            }
            if node.0 < SIZE {
                let east = Point(node.0 + 1, node.1);
                if !obstacles.contains(&east) {
                    neigh.push(east);
                }
            }
            if node.1 > 0 {
                let north = Point(node.0, node.1 - 1);
                if !obstacles.contains(&north) {
                    neigh.push(north);
                }
            }

            if node.0 > 0 {
                let west = Point(node.0 - 1, node.1);
                if !obstacles.contains(&west) {
                    neigh.push(west);
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
) -> Option<Vec<Point>> {
    let start = Point(0, 0);
    bfs(
        &start,
        #[inline]
        |node: &Point| {
            let mut neigh = SmallVec::<[_; 4]>::new();
            if node.1 < SIZE {
                let south = Point(node.0, node.1 + 1);
                if !obstacles.get(&south).is_some_and(|t| *t < time) {
                    neigh.push(south);
                }
            }
            if node.0 < SIZE {
                let east = Point(node.0 + 1, node.1);
                if !obstacles.get(&east).is_some_and(|t| *t < time) {
                    neigh.push(east);
                }
            }
            if node.1 > 0 {
                let north = Point(node.0, node.1 - 1);
                if !obstacles.get(&north).is_some_and(|t| *t < time) {
                    neigh.push(north);
                }
            }

            if node.0 > 0 {
                let west = Point(node.0 - 1, node.1);
                if !obstacles.get(&west).is_some_and(|t| *t < time) {
                    neigh.push(west);
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

#[aoc(day18, part2, union_find)]
fn two_union_find(points: &[Point]) -> String {
    #[cfg(test)]
    const SIZE: Coord = 6;
    #[cfg(not(test))]
    const SIZE: Coord = 70;

    #[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq)]
    enum Node {
        #[default]
        LowerLeft,
        Tile(Point),
        UpperRight,
    }

    impl Node {
        const MAX: usize = ((SIZE + 1) * (SIZE + 1)) as usize - 1;

        #[inline]
        fn neighbors(&self) -> SmallVec<[Self; 8]> {
            match self {
                Node::Tile(point) => {
                    let mut neigh = SmallVec::new();
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
                        let p = Point(
                            point.0.wrapping_add_signed(dx),
                            point.1.wrapping_add_signed(dy),
                        );
                        if (p.0 > 0 || p.1 > 0)
                            && (p.0 < SIZE || p.1 < SIZE)
                            && (p.0 <= SIZE && p.1 <= SIZE)
                        {
                            neigh.push(Node::Tile(p))
                        }
                    }
                    if point.0 == 0 || point.1 == SIZE {
                        neigh.push(Node::LowerLeft);
                    } else if point.0 == SIZE || point.1 == 0 {
                        neigh.push(Node::UpperRight);
                    }
                    neigh
                }
                _ => SmallVec::new(),
            }
        }
    }

    impl PartialOrd for Node {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Node {
        #[inline]
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (Node::LowerLeft, Node::LowerLeft) => Ordering::Equal,
                (Node::LowerLeft, _) => Ordering::Less,
                (Node::Tile(_), Node::LowerLeft) => Ordering::Greater,
                (Node::Tile(point), Node::Tile(other)) => {
                    (point.1, point.0).cmp(&(other.1, other.0))
                }
                (Node::Tile(_), Node::UpperRight) => Ordering::Less,
                (Node::UpperRight, Node::UpperRight) => Ordering::Equal,
                (Node::UpperRight, _) => Ordering::Greater,
            }
        }
    }

    unsafe impl IndexType for Node {
        #[inline]
        fn new(x: usize) -> Self {
            match x {
                0 => Node::LowerLeft,
                Self::MAX => Node::UpperRight,
                x @ _ => {
                    let p = Point(x as Coord % (SIZE + 1), x as Coord / (SIZE + 1));
                    debug_assert!(p.0 > 0 || p.1 > 0, "Top-left corner is reserved");
                    debug_assert!(p.0 < SIZE || p.1 < SIZE, "Bottom-right corner is reserved");
                    debug_assert!(
                        p.0 <= SIZE && p.1 <= SIZE,
                        "Coordinates {p:?} should be within field"
                    );
                    Node::Tile(p)
                }
            }
        }

        #[inline]
        fn index(&self) -> usize {
            match self {
                Node::LowerLeft => 0,
                Node::Tile(point) => (point.0 + point.1 * (SIZE + 1)) as usize,
                Node::UpperRight => Self::MAX,
            }
        }

        #[inline]
        fn max() -> Self {
            Self::UpperRight
        }
    }

    let mut blockage: UnionFind<Node> = UnionFind::new(Node::MAX + 1);
    let mut has_dropped = vec![false; Node::MAX + 1];
    has_dropped[Node::LowerLeft.index()] = true;
    has_dropped[Node::UpperRight.index()] = true;
    for new_p in points {
        let node = Node::Tile(*new_p);
        for neighbor in node.neighbors() {
            if has_dropped[neighbor.index()] {
                blockage.union(node, neighbor);
            }
        }
        if blockage.equiv(Node::LowerLeft, Node::UpperRight) {
            return format!("{},{}", new_p.0, new_p.1);
        }
        has_dropped[node.index()] = true;
    }
    panic!("No solution found")
}

pub fn part1(puzzle: &str) -> usize {
    one(&parse(puzzle))
}

pub fn part2(puzzle: &str) -> String {
    two_union_find(&parse(puzzle))
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
    #[test]
    fn example2_uf() {
        let res = two_union_find(&parse(include_str!("test.txt")));
        assert_eq!(res, "6,1");
    }
}
