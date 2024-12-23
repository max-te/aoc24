use std::borrow::Cow;

use aoc_runner_derive::aoc;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use smallvec::{smallvec, SmallVec};

#[aoc(day23, part1)]
pub fn part1(puzzle: &str) -> u64 {
    let mut links: std::collections::HashMap<_, Vec<_>, _> = FxHashMap::default();
    for line in puzzle.lines() {
        let (a, b) = line.split_once('-').unwrap();
        links.entry(a).or_default().push(b);
        links.entry(b).or_default().push(a);
    }
    let mut count = 0;
    for (node, neighbors) in links.iter() {
        if node.starts_with('t') {
            for neighbor in neighbors {
                if neighbor.starts_with('t') && neighbor > node {
                    continue;
                }

                let second_neighbors = links.get(neighbor).unwrap();
                for second_neighbor in second_neighbors {
                    if second_neighbor > neighbor {
                        continue;
                    }
                    if second_neighbor.starts_with('t') && second_neighbor > node {
                        continue;
                    }

                    if neighbors.contains(second_neighbor) {
                        #[cfg(debug_assertions)]
                        println!("{node}-{neighbor}-{second_neighbor}");
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
    let mut links: std::collections::HashMap<_, Vec<_>, _> = FxHashMap::default();
    for line in puzzle.lines() {
        let (a, b) = line.split_once('-').unwrap();
        links.entry(a).or_default().push(b);
        links.entry(b).or_default().push(a);
    }

    let nodes_descending_degree = links.iter().sorted_by_key(|x| x.1.len()).collect_vec();

    let mut largest_clique = SmallVec::new();
    for (node, neighbors) in nodes_descending_degree {
        // eprintln!("{node} {neighbors:?} ? {}", largest_clique.len());
        if neighbors.len() < largest_clique.len() {
            continue;
        }
        if let Some(clique) = find_clique_larger_than(
            &smallvec![*node],
            neighbors,
            &links,
            largest_clique.len() + 1,
        ) {
            // eprintln!("Better: {clique:?} {}", clique.len());
            largest_clique = clique;
        }
    }

    largest_clique.sort();
    largest_clique.iter().join(",")
}

type Node<'i> = &'i str;

fn find_clique_larger_than<'a, 'i>(
    current_clique: &'a SmallVec<[Node<'i>; 10]>,
    additional_node_pool: &'a [Node<'i>],
    links: &'a FxHashMap<Node, Vec<Node<'i>>>,
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
        new_clique.push(node);
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
