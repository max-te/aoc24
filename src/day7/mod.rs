use aoc_runner_derive::{aoc, aoc_generator};

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
    #[inline]
    fn apply(&self, a: Num, b: Num) -> Num {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Con => {
                let b_digits = b.checked_ilog10().unwrap_or(0) + 1;
                a * (10 as Num).pow(b_digits) + b
            }
        }
    }
}

#[aoc_generator(day7)]
fn parse(puzzle: &str) -> Input {
    puzzle
        .lines()
        .map(|l| {
            let (res, parts) = l.split_once(": ").unwrap();
            (
                res.parse().unwrap(),
                parts.split(' ').map(|s| s.parse().unwrap()).collect(),
            )
        })
        .collect()
}

#[aoc(day7, part1)]
fn one(input: &Input) -> Output {
    let mut answer = 0;
    for (res, numbers) in input {
        if mul_solvable(*res, numbers[0], &numbers[1..]) {
            answer += res;
        }
    }
    answer
}

fn mul_solvable(res: Num, first_value: Num, term: &[Num]) -> bool {
    if first_value == res && term.is_empty() {
        true
    } else if first_value > res {
        return false;
    } else if !term.is_empty() {
        let num = term[0];
        return mul_solvable(res, Op::Add.apply(first_value, num), &term[1..])
            || mul_solvable(res, Op::Mul.apply(first_value, num), &term[1..]);
    } else {
        false
    }
}

fn con_mul_solvable(res: Num, first_value: Num, term: &[Num]) -> bool {
    if first_value == res && term.is_empty() {
        true
    } else if first_value > res {
        return false;
    } else if !term.is_empty() {
        let num = term[0];
        return con_mul_solvable(res, Op::Add.apply(first_value, num), &term[1..])
            || con_mul_solvable(res, Op::Con.apply(first_value, num), &term[1..])
            || con_mul_solvable(res, Op::Mul.apply(first_value, num), &term[1..]);
    } else {
        false
    }
}

#[aoc(day7, part2)]
fn two(input: &Input) -> Output {
    let mut answer = 0;
    for (res, numbers) in input {
        if con_mul_solvable(*res, numbers[0], &numbers[1..]) {
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
