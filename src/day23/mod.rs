use std::{borrow::Cow, collections::HashMap};

use aoc_runner_derive::aoc;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use smallvec::{smallvec, SmallVec};

type Node<'i> = [u8; 2];

#[aoc(day23, part1)]
pub fn part1(puzzle: &str) -> u64 {
    let mut links: HashMap<Node, SmallVec<[Node; 10]>, _> = FxHashMap::default();
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
    let mut links: HashMap<Node, SmallVec<[Node; 10]>, _> = FxHashMap::default();
    for line in puzzle.lines() {
        let line = line.as_bytes();
        let a = [line[0], line[1]];
        let b = [line[3], line[4]];
        links.entry(a).or_default().push(b);
        links.entry(b).or_default().push(a);
    }

    let nodes_descending_degree = links.iter().sorted_by_key(|x| x.1.len()).collect_vec();

    let mut largest_clique = smallvec![
        *nodes_descending_degree[0].0,
        nodes_descending_degree[0].1[0]
    ];
    for (node, neighbors) in nodes_descending_degree {
        // eprintln!("{node} {neighbors:?} ? {}", largest_clique.len());
        if neighbors.len() < largest_clique.len() {
            continue;
        }
        if let Some(clique) = find_clique_larger_than(
            &smallvec![*node],
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
    current_clique: &'a SmallVec<[Node<'i>; 10]>,
    additional_node_pool: &'a [Node<'i>],
    links: &'a FxHashMap<Node, SmallVec<[Node<'i>; 10]>>,
    min_size: usize,
) -> Option<SmallVec<[Node<'i>; 10]>> {
    // eprintln!("find_clique_larger_than({current_clique:?}, {additional_node_pool:?}, {min_size})");
    if current_clique.len() + additional_node_pool.len() < min_size {
        // eprintln!("find_clique_larger_than({current_clique:?}, {additional_node_pool:?}, {min_size}) = None (fast)");
        return None;
    }
    let mut best_clique = Cow::Borrowed(current_clique);
    'pool: for (i, node) in additional_node_pool.iter().enumerate() {
        let neighbors = links.get(node).unwrap();
        if neighbors.len() < min_size {
            continue 'pool;
        }
        for c in current_clique {
            if !neighbors.contains(c) {
                continue 'pool;
            }
        }
        let mut new_clique = current_clique.clone();
        new_clique.push(*node);
        let better_clique =
            find_clique_larger_than(&new_clique, &additional_node_pool[i + 1..], links, min_size);
        if let Some(better_clique) = better_clique {
            if better_clique.len() > best_clique.len() {
                best_clique = Cow::Owned(better_clique);
            }
        }
        if new_clique.len() > best_clique.len() {
            best_clique = Cow::Owned(new_clique);
        }
    }
    if best_clique.len() >= min_size {
        // eprintln!("find_clique_larger_than({current_clique:?}, {additional_node_pool:?}, {min_size}) = Some({best_clique:?})");
        Some(best_clique.into_owned())
    } else {
        // eprintln!("find_clique_larger_than({current_clique:?}, {additional_node_pool:?}, {min_size}) = None");
        None
    }
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
