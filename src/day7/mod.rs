use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Num = u64;
type Input = Vec<(Num, Vec<Num>)>;
type Output = Num;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Mul => write!(f, "*"),
        }
    }
}

type Term = Vec<(Op, Num)>;

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
    for (res, numbers) in input {
        let term = numbers.iter().map(|i| {
            assert!(*i > 0);
            (Op::Add, *i)
        }).collect::<Vec<_>>();
        if solvable(*res, &term, 1) {
            answer += res;
        }
    }
    answer
}

fn solvable(res: Num, term: &Term, fixed_until: usize) -> bool {
    let current_res = calculate(&term);
    let _debug_term = term.iter().map(|(op, num)| format!(" {} {}", op, num)).join("");
    if current_res == res {
        // eprintln!("{_debug_term} = {res}");
        true
    } else if current_res > res && calculate(&term[..fixed_until]) != 1 {
        // eprintln!("{_debug_term} > {res}");
        let ones = term.iter().enumerate().skip(fixed_until).filter(|(_i, part)| part.1 == 1 && part.0 == Op::Add).map(|i| i.0).collect::<Vec<_>>();
        if ones.len() as Num >= current_res - res {
            for i in ones {
                let mut term = term.clone();
                term[i].0 = Op::Mul;
                if solvable(res, &term, fixed_until) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    } else {
        // eprintln!("{_debug_term} < {res}");
        for i in fixed_until..term.len() {
            if term[i].0 == Op::Mul {
                continue
            }
            let mut term = term.clone();
            term[i].0 = Op::Mul;
            if solvable(res, &term, i + 1) {
                return true;
            }
        }
        false
    }
}

fn calculate(term: &[(Op, Num)]) -> Num {
    let init = match term[0].0 {
        Op::Add => 0,
        Op::Mul => 1,
    };
    term.iter().fold(init, |acc, (op, n)| match op {
        Op::Add => acc.checked_add(*n).unwrap(),
        Op::Mul => acc.checked_mul(*n).unwrap(),
    })
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
