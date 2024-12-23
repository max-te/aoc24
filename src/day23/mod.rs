use std::collections::HashMap;

use aoc_runner_derive::aoc;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use smallvec::{smallvec, SmallVec};

type Node<'i> = [u8; 2];
const TYPICAL_DEGREE: usize = 13;

#[aoc(day23, part1)]
pub fn part1(puzzle: &str) -> u64 {
    let mut links: HashMap<Node, SmallVec<[Node; TYPICAL_DEGREE]>, _> = FxHashMap::default();
    for line in puzzle.lines() {
        let line = line.as_bytes();
        let a = [line[0], line[1]];
        let b = [line[3], line[4]];
        links.entry(a).or_default().push(b);
        links.entry(b).or_default().push(a);
    }
    let mut count = 0;
    for (node, neighbors) in links.iter() {
        if node[0] == b't' {
            for neighbor in neighbors {
                if neighbor[0] == b't' && neighbor > node {
                    continue;
                }

                let second_neighbors = links.get(neighbor).unwrap();
                for second_neighbor in second_neighbors {
                    if second_neighbor > neighbor {
                        continue;
                    }
                    if second_neighbor[0] == b't' && second_neighbor > node {
                        continue;
                    }

                    if neighbors.contains(second_neighbor) {
                        #[cfg(debug_assertions)]
                        println!("{node:?}-{neighbor:?}-{second_neighbor:?}");
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[aoc(day23, part2)]
pub fn part2(puzzle: &str) -> String {
    let mut links: HashMap<Node, SmallVec<[Node; TYPICAL_DEGREE]>, _> = FxHashMap::default();
    for line in puzzle.lines() {
        let line = line.as_bytes();
        let a = [line[0], line[1]];
        let b = [line[3], line[4]];
        links.entry(a).or_default().push(b);
        links.entry(b).or_default().push(a);
    }
    for neigh in links.values_mut() {
        neigh.sort_unstable();
    }

    let nodes_sorted = links
        .iter()
        .sorted_unstable_by_key(|x| x.1.len())
        .collect_vec();

    let mut largest_clique = smallvec![*nodes_sorted[0].0, nodes_sorted[0].1[0]];
    for (node, neighbors) in nodes_sorted {
        // eprintln!("{node} {neighbors:?} ? {}", largest_clique.len());
        if neighbors.len() < largest_clique.len() {
            continue;
        }
        if let Some(clique) = find_clique_larger_than(
            &mut smallvec![*node],
            neighbors.as_slice(),
            &links,
            largest_clique.len() + 1,
        ) {
            // eprintln!("Better: {clique:?} {}", clique.len());
            largest_clique = clique;
        }
    }

    largest_clique.sort();
    let largest_clique = largest_clique
        .iter()
        .flat_map(|x| [b',', x[0], x[1]])
        .collect_vec();
    String::from_utf8_lossy(&largest_clique[1..]).to_string()
}

fn find_clique_larger_than<'a, 'i>(
    current_clique: &'a mut SmallVec<[Node<'i>; TYPICAL_DEGREE]>,
    additional_node_pool: &'a [Node<'i>],
    links: &'a FxHashMap<Node, SmallVec<[Node<'i>; TYPICAL_DEGREE]>>,
    min_size: usize,
) -> Option<SmallVec<[Node<'i>; TYPICAL_DEGREE]>> {
    if current_clique.len() + additional_node_pool.len() < min_size {
        return None;
    }
    let mut best_clique: Option<SmallVec<[Node<'i>; TYPICAL_DEGREE]>> = None;
    for (i, node) in additional_node_pool.iter().enumerate() {
        let neighbors = links.get(node).unwrap();
        if neighbors.len() < min_size {
            continue;
        }
        if !sorted_is_subset(&current_clique[1..], neighbors) {
            continue;
        }
        current_clique.push(*node);
        let better_clique = find_clique_larger_than(
            current_clique,
            &additional_node_pool[i + 1..],
            links,
            min_size,
        );

        if let Some(better_clique) = better_clique {
            if best_clique.as_ref().map(|x| x.len()).unwrap_or(0) < better_clique.len() {
                best_clique = Some(better_clique);
            }
        }
        current_clique.pop();
    }
    if best_clique
        .as_ref()
        .map(|x| x.len() >= min_size)
        .unwrap_or(false)
    {
        best_clique
    } else if current_clique.as_ref().len() >= min_size {
        Some(current_clique.clone())
    } else {
        None
    }
}

fn sorted_is_subset(sub: &[Node], sup: &[Node]) -> bool {
    let mut cur_idx = 0;
    for s in sub {
        while s > &sup[cur_idx] {
            cur_idx += 1;
            if cur_idx == sup.len() {
                return false;
            }
        }
        if s == &sup[cur_idx] {
            cur_idx += 1;
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");
        assert_eq!(part2(input), "co,de,ka,ta");
    }
}
