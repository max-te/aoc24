use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::{graph::NodeIndex, visit::EdgeRef, Graph};
use rustc_hash::FxHashSet;
use smallvec::{smallvec, SmallVec};

use crate::util::first_line_length;

type Num = u32;
type Input = (Graph<(), Num>, NodeIndex, [NodeIndex; 4], NodeIndex, usize);

#[aoc_generator(day16)]
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

#[aoc(day16, part1, dir_dijkstra)]
fn one((maze, start, _, goal, _): &Input) -> Num {
    let path = petgraph::algo::dijkstra(&maze, *start, Some(*goal), |e| *e.weight());
    *path.get(goal).unwrap() - 1
}

#[aoc(day16, part1, dir_astar)]
fn one_astar((maze, start, end, goal, width): &Input) -> Num {
    let goal_base_idx = end[0].index() / 4;
    let path = petgraph::algo::astar(
        maze,
        *start,
        |v| v == *goal,
        |e| *e.weight(),
        |v| {
            if v == *goal {
                0
            } else {
                let idx_diff = goal_base_idx.abs_diff(v.index() / 4);
                let h_diff = idx_diff % width;
                let v_diff = idx_diff / width;
                (h_diff + v_diff) as u32 + {
                    if h_diff > 0 && v_diff > 0 {
                        1000
                    } else {
                        0
                    }
                }
            }
        },
    );
    path.unwrap().0 - 1
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

pub fn part1(puzzle: &str) -> Num {
    one(&parse(puzzle))
}

pub fn part2(puzzle: &str) -> usize {
    two(&parse(puzzle))
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
