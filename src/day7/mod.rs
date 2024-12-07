use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::util::GroupedView;

type Num = u32;
type Input = Vec<(Num, Vec<Num>)>;
type Output = Num;

#[aoc_generator(day7)]
fn parse(puzzle: &str) -> Input {
    puzzle.lines().map(|l| {
        let (res, parts) = l.split_once(": ").unwrap();
        (res.parse().unwrap(), parts.split(' ').map(|s| s.parse().unwrap()).collect())
    }).collect()
}

#[aoc(day7, part1)]
fn one(input: &Input) -> Output {
    let mut answer = 0;
    for (res, parts) in input {
        let view = GroupedView::of_singletons(&parts);
        if solvable(*res, &view) {
            answer += res;
        }
    }
    answer
}

fn solvable<'i>(res: Num, parts: &GroupedView<'i, Num>) -> bool {
    let current_res = mul_sum(&parts);
    let _debug_term = parts.iter().map(|part| part.iter().map(|n| n.to_string()).join("*") ).join(" + ");
    if current_res == res {
        eprintln!("{_debug_term} = {res}");
        true
    } else if current_res > res {
        eprintln!("{_debug_term} > {res}");
        let ones = parts.iter().enumerate().filter(|(_i, part)| *part == &[1]).collect::<Vec<_>>();
        if ones.len() as Num >= current_res - res {
            todo!()
        } else {
            false
        }
    } else {
        eprintln!("{_debug_term} < {res}");
        for i in 1..parts.len() {
            let mut parts = parts.clone();
            let merge_point = parts.merge_left(i);
            if solvable(res, &parts) {
                return true;
            }
            parts.split(i - 1, merge_point);
        }
        false
    }
}

fn mul_sum<'i>(parts: &GroupedView<'i, Num>) -> Num {
    // Assumption: no empty groups
    parts.iter().map(|part| part.iter().fold(1, |a, b| a * b)).sum()
}

#[aoc(day7, part2)]
fn two(input: &Input) -> Output {
    todo!()
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
        assert_eq!(res, 3749);
    }

    // #[test]
    // fn example2() {
    //     let res = part2(include_str!("test.txt"));
    //     assert_eq!(res, todo!());
    // }

}
