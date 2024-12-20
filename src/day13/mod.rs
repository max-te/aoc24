use core::str;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::parse_initial_digits;

type Num = i64;
#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    a_x: Num,
    a_y: Num,
    b_x: Num,
    b_y: Num,
    target_x: Num,
    target_y: Num,
}

type Input = Vec<ClawMachine>;
type Output = usize;

const BUTTON_X_LEN: usize = "Button _: X+".len();
const PRIZE_LEN: usize = "Prize: X=".len();

#[aoc_generator(day13)]
fn parse(input: &str) -> Input {
    let mut input = input.as_bytes();
    let mut machines = Vec::with_capacity(input.len() / 64);
    loop {
        input = &input[BUTTON_X_LEN..];
        let (a_x, num_len) = parse_initial_digits(&input);
        input = &input[num_len + 4..];
        let (a_y, num_len) = parse_initial_digits(&input);
        input = &input[num_len + 1..];

        input = &input[BUTTON_X_LEN..];
        let (b_x, num_len) = parse_initial_digits(&input);
        input = &input[num_len + 4..];
        let (b_y, num_len) = parse_initial_digits(&input);
        input = &input[num_len + 1..];

        input = &input[PRIZE_LEN..];
        let (target_x, num_len) = parse_initial_digits(&input);
        input = &input[num_len + 4..];
        let (target_y, num_len) = parse_initial_digits(&input);
        input = &input[num_len..];

        machines.push(ClawMachine {
            a_x,
            a_y,
            b_x,
            b_y,
            target_x,
            target_y,
        });

        if input.len() < 2 {
            break;
        } else {
            input = &input[2..];
        }
    }
    machines
}

fn solve_linear(m: &ClawMachine) -> Option<(Num, Num)> {
    let det = m.a_x * m.b_y - m.a_y * m.b_x;
    if det == 0 {
        #[cfg(debug_assertions)]
        eprintln!("System {m:?} is degenerate, might still be solvable if input is evil");
        None
    } else {
        let a = (m.target_x * m.b_y) - (m.target_y * m.b_x);
        let b = (m.target_y * m.a_x) - (m.target_x * m.a_y);
        if a % det != 0 || b % det != 0 {
            None
        } else {
            Some((a / det, b / det))
        }
    }
}

#[aoc(day13, part1)]
fn one(machines: &Input) -> Output {
    let mut tokens = 0;
    for machine in machines {
        let s = solve_linear(machine);
        // dbg!(&machine, &s);
        if let Some((a, b)) = s {
            if a >= 0 && b >= 0 {
                // eprintln!("Found solution {a} {b}");
                tokens += 3 * (a as usize) + (b as usize);
            }
        }
    }
    tokens
}

#[aoc(day13, part2)]
fn two(machines: &Input) -> Output {
    let offset: Num = 10000000000000;
    let mut tokens = 0;
    for machine in machines {
        let mut machine = machine.clone();
        machine.target_x += offset;
        machine.target_y += offset;
        let s = solve_linear(&machine);
        // dbg!(&machine, &s);
        if let Some((a, b)) = s {
            if a >= 0 && b >= 0 {
                // eprintln!("Found solution {a} {b}");
                tokens += 3 * (a as usize) + (b as usize);
            }
        }
    }
    tokens
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
        let res = one(&parse(include_str!("test.txt")));
        assert_eq!(res, 480);
    }

    // #[test]
    // fn example2() {
    //     let res = two(&parse(include_str!("test.txt")));
    //     assert_eq!(res, todo!());
    // }
}
