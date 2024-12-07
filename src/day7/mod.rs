use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Num = u64;
type Input = Vec<(Num, Vec<Num>)>;
type Output = Num;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Con,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Mul => write!(f, "*"),
            Op::Con => write!(f, "|"),
        }
    }
}

impl Op {
    fn apply(&self, a: Num, b: Num) -> Num {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Con => {
                let b_digits = b.checked_ilog10().unwrap_or(0) + 1;
                a * (10 as Num).pow(b_digits) + b
            },
        }
    }

    fn identity(self) -> Num {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
            Op::Con => 0,
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
        let mut term = numbers.iter().map(|i| {
            assert!(*i > 0);
            (Op::Add, *i)
        }).collect::<Vec<_>>();
        if mul_solvable(*res, &mut term, 1) {
            answer += res;
        }
    }
    answer
}

fn mul_solvable(res: Num, term: &mut Term, fixed_until: usize) -> bool {
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
                term[i].0 = Op::Mul;
                if mul_solvable(res, term, fixed_until) {
                    return true;
                }
                term[i].0 = Op::Add;
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
            term[i].0 = Op::Mul;
            if mul_solvable(res, term, i + 1) {
                return true;
            }
            term[i].0 = Op::Add;
        }
        false
    }
}

fn con_mul_solvable(res: Num, term: &mut Term, fixed_until: usize) -> bool {
    let current_res = calculate(&term);
    // let _debug_term = term.iter().map(|(op, num)| format!(" {} {}", op, num)).join("");
    if current_res == res {
        // eprintln!("{_debug_term} = {res}");
        true
    } else if current_res > res && calculate(&term[..fixed_until]) != 1 {
        // eprintln!("{_debug_term} > {res}");
        let ones = term.iter().enumerate().skip(fixed_until).filter(|(_i, part)| part.1 == 1 && part.0 == Op::Add).map(|i| i.0).collect::<Vec<_>>();
        if ones.len() as Num >= current_res - res {
            for i in ones {
                term[i].0 = Op::Mul;
                if con_mul_solvable(res, term, fixed_until) {
                    return true;
                }
                term[i].0 = Op::Add;
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
            term[i].0 = Op::Mul;
            if con_mul_solvable(res, term, i + 1) {
                return true;
            }
            term[i].0 = Op::Con;
            if con_mul_solvable(res, term, i + 1) {
                return true;
            }
            term[i].0 = Op::Add;
        }
        false
    }
}

fn calculate(term: &[(Op, Num)]) -> Num {
    let init = term[0].0.identity();
    term.iter().fold(init, |acc, (op, n)| op.apply(acc, *n))
}

#[aoc(day7, part2)]
fn two(input: &Input) -> Output {
    let mut answer = 0;
    for (res, numbers) in input {
        let mut term = numbers.iter().map(|i| {
            assert!(*i > 0);
            (Op::Add, *i)
        }).collect::<Vec<_>>();
        if con_mul_solvable(*res, &mut term, 1) {
            answer += res;
        }
    }
    answer
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

    #[test]
    fn example2() {
        let res = part2(include_str!("test.txt"));
        assert_eq!(res, 11387);
    }

}
