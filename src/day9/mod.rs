use crate::util::parse_digit;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Output = usize;
type Input = Vec<Output>;

#[aoc_generator(day9)]
fn parse(puzzle: &str) -> Input {
    let puzzle = puzzle.as_bytes();
    puzzle
        .iter()
        .filter(|ch| **ch != b'\n')
        .map(parse_digit)
        .map(|i| i as Output)
        .collect()
}

#[aoc(day9, part1, naive)]
fn one(map: &Input) -> Output {
    let drive_size = map.iter().sum::<Output>() as usize;
    let mut assignments = Vec::with_capacity(drive_size);
    let mut last_filled: usize = 0;
    for (file_id, mut sizes) in map.into_iter().chunks(2).into_iter().enumerate() {
        let file_size = *(sizes.next().unwrap());
        for _ in 0..file_size {
            last_filled = assignments.len();
            assignments.push(file_id);
        }
        let space_after = *(sizes.next().unwrap_or(&0));
        for _ in 0..space_after {
            assignments.push(usize::MAX);
        }
    }

    let mut first_empty = 0;
    loop {
        while assignments[first_empty] != usize::MAX {
            first_empty += 1;
        }
        while assignments[last_filled] == usize::MAX {
            last_filled -= 1;
        }
        if first_empty >= last_filled {
            break;
        }
        assignments.swap(first_empty, last_filled);
    }

    let mut checksum = 0;
    for i in 0..assignments.len() {
        if assignments[i] == usize::MAX {
            break;
        }
        checksum += i * assignments[i];
    }

    checksum
}

#[aoc(day9, part1, one_pass)]
fn one_pass(map: &Input) -> Output {
    let mut left_idx = 0;
    let mut right_idx = map.len() - 1;
    if right_idx % 2 == 1 {
        // Odd index = space
        right_idx -= 1;
    }
    let mut right_file_id = right_idx / 2;
    let mut right_file_size_remain = map[right_idx];
    let mut write_pos = 0;
    let mut checksum = 0;

    while right_idx > left_idx {
        if left_idx % 2 == 0 {
            let left_file_id = left_idx / 2;
            let left_file_size = map[left_idx];
            checksum += checksum_summand(left_file_id, write_pos, left_file_size);
            // eprint!("L{left_file_size}x{left_file_id} ");
            write_pos += left_file_size;
            left_idx += 1;
        } else {
            let mut left_space_size = map[left_idx];
            while left_space_size > 0 {
                if right_file_size_remain <= left_space_size {
                    checksum += checksum_summand(right_file_id, write_pos, right_file_size_remain);
                    // eprint!("r{right_file_size_remain}x{right_file_id} ");
                    write_pos += right_file_size_remain;
                    left_space_size -= right_file_size_remain;
                    right_idx -= 2;
                    right_file_id = right_idx / 2;
                    right_file_size_remain = map[right_idx];
                } else {
                    checksum += checksum_summand(right_file_id, write_pos, left_space_size);
                    // eprint!("R{left_space_size}x{right_file_id} ");
                    write_pos += left_space_size;
                    right_file_size_remain -= left_space_size;
                    left_space_size = 0;
                }
            }
            left_idx += 1;
        }
    }

    checksum += checksum_summand(right_file_id, write_pos, right_file_size_remain);
    // eprint!("+{right_file_size_remain}x{right_file_id} ");

    checksum
}

#[inline(always)]
fn checksum_summand(file_id: usize, from_pos: usize, block_count: usize) -> usize {
    file_id * (from_pos * block_count + block_count * (block_count - 1) / 2)
}

#[aoc(day9, part2)]
fn two(map: &Input) -> Output {
    let mut files = Vec::new();
    let mut spaces = Vec::new();
    let mut pos = 0;
    for (file_id, mut sizes) in map.into_iter().chunks(2).into_iter().enumerate() {
        let file_size = *(sizes.next().unwrap());
        files.push((file_id, pos, file_size));
        pos += file_size;
        let space_after = *(sizes.next().unwrap_or(&0));
        spaces.push((pos, space_after));
        pos += space_after;
    }

    for (_file_id, file_pos, file_size) in files.iter_mut().rev() {
        let mut target_space = None;
        for (idx, (space_pos, space_size)) in spaces.iter_mut().enumerate() {
            if space_pos > file_pos {
                break;
            }
            if space_size >= file_size {
                target_space = Some(idx);
                break;
            }
        }
        if let Some(idx) = target_space {
            let (space_pos, space_size) = &mut spaces[idx];
            *file_pos = *space_pos;

            if space_size > file_size {
                *space_pos += *file_size;
                *space_size -= *file_size;
            } else {
                spaces.remove(idx);
            }
        }
    }
    let mut checksum = 0;
    for (file_id, file_pos, file_size) in files.iter() {
        checksum += checksum_summand(*file_id, *file_pos, *file_size);
    }

    checksum
}

pub fn part1(puzzle: &str) -> Output {
    one_pass(&parse(puzzle))
}

pub fn part2(puzzle: &str) -> Output {
    two(&parse(puzzle))
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = one(&parse(include_str!("test.txt")));
        assert_eq!(res, 1928);
    }

    #[test]
    fn example2() {
        let res = part2(include_str!("test.txt"));
        assert_eq!(res, 2858);
    }

    #[test]
    fn example1_onepass() {
        let res = one_pass(&parse(include_str!("test.txt")));
        assert_eq!(res, 1928);
    }
}
