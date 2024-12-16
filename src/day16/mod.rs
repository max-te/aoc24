use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::{graph::NodeIndex, visit::EdgeRef, Graph};
use rustc_hash::FxHashSet;
use smallvec::{smallvec, SmallVec};

use crate::util::first_line_length;

type Num = u32;
type Input = (Graph<(), Num>, NodeIndex, [NodeIndex; 4], NodeIndex, usize);

#[aoc_generator(day16, part1, dijkstra)]
#[aoc_generator(day16, part2, dijkstra)]
fn parse(input: &str) -> Input {
    let input = input.as_bytes().trim_ascii();
    let mut maze = Graph::<(), Num>::new();
    let width = first_line_length(input);
    const NODES_PER_SQUARE: usize = 4;
    let stride = width * NODES_PER_SQUARE;
    let mut col: usize = 0;
    let mut row: usize = 0;
    let mut start: Option<NodeIndex> = None;
    let mut end: Option<[NodeIndex; NODES_PER_SQUARE]> = None;
    maze.reserve_nodes(input.len() * NODES_PER_SQUARE);
    maze.reserve_edges(input.len() * NODES_PER_SQUARE * 3);
    let mut node_list = Vec::with_capacity(input.len() * NODES_PER_SQUARE);

    for &ch in input {
        if ch == b'\n' {
            col = 0;
            row += 1;
            continue;
        }
        if ch == b'#' {
            col += 1;
            node_list.extend_from_slice(&[None, None, None, None]);
            continue;
        }

        let v_idx_start = node_list.len();
        let v_north = maze.add_node(());
        let v_east = maze.add_node(());
        let v_south = maze.add_node(());
        let v_west = maze.add_node(());
        node_list.extend_from_slice(&[Some(v_north), Some(v_east), Some(v_south), Some(v_west)]);

        match ch {
            b'.' => {}
            b'S' => {
                start = Some(v_east);
            }
            b'E' => {
                end = Some([v_north, v_east, v_south, v_west]);
            }
            _ => unreachable!("invalid input character: {}", ch),
        }

        maze.extend_with_edges([
            (v_north, v_east, 1000),
            (v_east, v_north, 1000),
            (v_east, v_south, 1000),
            (v_south, v_east, 1000),
            (v_south, v_west, 1000),
            (v_west, v_south, 1000),
            (v_west, v_north, 1000),
            (v_north, v_west, 1000),
        ]);

        if row > 0 {
            if let Some(n_north) = node_list[v_idx_start - stride] {
                let n_south = node_list[v_idx_start - stride + 2].unwrap();
                maze.extend_with_edges([(v_north, n_north, 1), (n_south, v_south, 1)]);
            }
        }
        if col > 0 {
            if let Some(w_west) = node_list[v_idx_start - NODES_PER_SQUARE + 3] {
                let w_east = node_list[v_idx_start - NODES_PER_SQUARE + 1].unwrap();
                maze.extend_with_edges([(v_west, w_west, 1), (w_east, v_east, 1)]);
            }
        }

        col += 1;
    }
    let start = start.unwrap();
    let end = end.unwrap();
    let goal = maze.add_node(());
    maze.extend_with_edges([
        (end[0], goal, 1),
        (end[1], goal, 1),
        (end[2], goal, 1),
        (end[3], goal, 1),
    ]);

    (maze, start, end, goal, width)
}

#[aoc(day16, part1, dijkstra)]
fn one((maze, start, _, goal, _): &Input) -> Num {
    let path = petgraph::algo::dijkstra(&maze, *start, Some(*goal), |e| *e.weight());
    *path.get(goal).unwrap() - 1
}

#[aoc(day16, part2, dijkstra)]
fn two((maze, start, _, goal, _): &Input) -> usize {
    let distances = petgraph::algo::dijkstra(&maze, *start, None, |e| *e.weight());
    let mut nodes_on_paths: FxHashSet<NodeIndex> = FxHashSet::default();
    let mut front: SmallVec<[(NodeIndex, Num); 128]> =
        smallvec![(*goal, *distances.get(goal).unwrap())];
    while !front.is_empty() {
        let (node, dist) = front.pop().unwrap();
        if node != *goal {
            nodes_on_paths.insert(node);
        }

        for in_edge in maze.edges_directed(node, petgraph::Direction::Incoming) {
            let neighbor = in_edge.source();
            let e_len = *in_edge.weight();
            let n_dist = *distances.get(&neighbor).unwrap();
            // dbg!(&node, &neighbor, &n_dist, &e_len, &dist);
            if n_dist + e_len == dist {
                front.push((neighbor, n_dist));
            }
        }
    }
    let fields_on_paths = nodes_on_paths
        .iter()
        .map(|n| n.index() / 4)
        .collect::<FxHashSet<_>>();
    fields_on_paths.len()
}

pub fn part1(puzzle: &str) -> usize {
    one_astar(&parse_alt(puzzle))
}

pub fn part2(puzzle: &str) -> usize {
    two(&parse(puzzle))
}

#[derive(Debug, Clone)]
struct Grid2D {
    data: Vec<u8>,
    pad: usize,
    width: usize,
    height: usize,
}

impl Grid2D {
    fn new_from_newlines(data: Vec<u8>) -> Self {
        let width = first_line_length(&data);
        let pad = 1;
        let height = (data.len() + pad) / (width + pad);
        Self {
            data,
            pad,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.data[self.to_index(x, y)])
        }
    }

    fn is_passable(&self, x: usize, y: usize) -> bool {
        self.get(x, y).map_or(false, |c| c != b'#')
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        y * (self.width + self.pad) + x
    }

    fn to_point(&self, i: usize) -> (usize, usize) {
        (i % (self.width + self.pad), i / (self.width + self.pad))
    }
}

type InputPathfinding = (Grid2D, (usize, usize), (usize, usize));

#[aoc_generator(day16, part1, pathfinding)]
#[aoc_generator(day16, part1, pathfinding_astar)]
#[aoc_generator(day16, part2, pathfinding_astar)]
fn parse_alt(puzzle: &str) -> InputPathfinding {
    let grid = Grid2D::new_from_newlines(puzzle.as_bytes().to_vec());
    let mut start = None;
    let mut end = None;
    for i in 0..grid.data.len() {
        let c = grid.data[i];
        if c == b'S' {
            start = Some(grid.to_point(i));
            if end.is_some() {
                break;
            }
        } else if c == b'E' {
            end = Some(grid.to_point(i));
            if start.is_some() {
                break;
            }
        }
    }

    (grid, start.unwrap(), end.unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Facing {
    North,
    South,
    West,
    East,
}

impl Facing {
    #[inline]
    fn advance(&self, point: (usize, usize)) -> (usize, usize) {
        match self {
            Facing::North => (point.0, point.1 - 1),
            Facing::South => (point.0, point.1 + 1),
            Facing::West => (point.0 - 1, point.1),
            Facing::East => (point.0 + 1, point.1),
        }
    }

    #[inline]
    fn turn_right(&self) -> Facing {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }

    #[inline]
    fn turn_left(&self) -> Facing {
        match self {
            Facing::North => Facing::West,
            Facing::West => Facing::South,
            Facing::South => Facing::East,
            Facing::East => Facing::North,
        }
    }
}

#[aoc(day16, part1, pathfinding)]
fn one_alt((grid, start, end): &InputPathfinding) -> Num {
    let start = (start.0, start.1, Facing::East);
    let res = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |(x, y, d)| {
            let mut turns: SmallVec<[_; 3]> = smallvec![
                ((*x, *y, d.turn_left()), 1000),
                ((*x, *y, d.turn_right()), 1000),
            ];
            let forward = d.advance((*x, *y));
            if grid.is_passable(forward.0, forward.1) {
                turns.push(((forward.0, forward.1, *d), 1));
            }
            turns
        },
        |node| node.0 == end.0 && node.1 == end.1,
    )
    .unwrap();

    res.1
}

#[aoc(day16, part1, pathfinding_astar)]
fn one_astar((grid, start, end): &InputPathfinding) -> usize {
    let start = (start.0, start.1, Facing::East);
    let res = pathfinding::directed::astar::astar(
        &start,
        |(x, y, d)| {
            let mut turns: SmallVec<[_; 3]> = smallvec![
                ((*x, *y, d.turn_left()), 1000),
                ((*x, *y, d.turn_right()), 1000),
            ];
            let forward = d.advance((*x, *y));
            if grid.is_passable(forward.0, forward.1) {
                turns.push(((forward.0, forward.1, *d), 1));
            }
            turns
        },
        |node| node.0.abs_diff(end.0) + node.1.abs_diff(end.1),
        |node| node.0 == end.0 && node.1 == end.1,
    )
    .unwrap();

    res.1
}

#[aoc(day16, part2, pathfinding_astar)]
fn two_astar((grid, start, end): &InputPathfinding) -> usize {
    let start = (start.0, start.1, Facing::East);
    let res = pathfinding::directed::astar::astar_bag(
        &start,
        |(x, y, d)| {
            let mut turns: SmallVec<[_; 3]> = smallvec![
                ((*x, *y, d.turn_left()), 1000),
                ((*x, *y, d.turn_right()), 1000),
            ];
            let forward = d.advance((*x, *y));
            if grid.is_passable(forward.0, forward.1) {
                turns.push(((forward.0, forward.1, *d), 1));
            }
            turns
        },
        |node| {
            node.0.abs_diff(end.0)
                + node.1.abs_diff(end.1)
                + if node.0 != end.0 && node.1 != end.1 {
                    1000
                } else {
                    0
                }
        },
        |node| node.0 == end.0 && node.1 == end.1,
    )
    .unwrap();

    let mut tiles_on_path: FxHashSet<(usize, usize)> = FxHashSet::default();
    for path in res.0 {
        tiles_on_path.extend(path.into_iter().map(|(x, y, _)| (x, y)));
    }
    tiles_on_path.len()
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, 7036);
    }

    #[test]
    fn example1b() {
        let res = part1(include_str!("test2.txt"));
        assert_eq!(res, 11048);
    }

    #[test]
    fn example2() {
        let res = part2(include_str!("test.txt"));
        assert_eq!(res, 45);
    }

    #[test]
    fn example2b() {
        let res = part2(include_str!("test2.txt"));
        assert_eq!(res, 64);
    }
}
