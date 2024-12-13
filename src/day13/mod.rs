use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num_rational::Rational64;

type Num = Rational64;
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

#[aoc_generator(day13)]
fn parse(input: &str) -> Input {
    let mut machines = Vec::new();
    for mut chunk in &input.lines().chunks(4) {
        let (button_a_1, button_a_2) = chunk
            .next()
            .unwrap()
            .strip_prefix("Button A: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let a_x: Num = button_a_1.parse().unwrap();
        let a_y: Num = button_a_2.parse().unwrap();
        let (button_b_1, button_b_2) = chunk
            .next()
            .unwrap()
            .strip_prefix("Button B: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let b_x: Num = button_b_1.parse().unwrap();
        let b_y: Num = button_b_2.parse().unwrap();
        let (target_1, target_2) = chunk
            .next()
            .unwrap()
            .strip_prefix("Prize: X=")
            .unwrap()
            .split_once(", Y=")
            .unwrap();
        let target_x: Num = target_1.parse().unwrap();
        let target_y: Num = target_2.parse().unwrap();

        machines.push(ClawMachine {
            a_x,
            a_y,
            b_x,
            b_y,
            target_x,
            target_y,
        });
    }
    machines
}

fn solve_linear(m: &ClawMachine) -> Option<(Num, Num)> {
    if (m.a_x * m.b_y - m.a_y * m.b_x) == Num::ZERO {
        eprintln!("System {m:?} is degenerate");
        None
    } else {
        let b = (m.target_y / m.a_y - m.target_x / m.a_x) / (m.b_y / m.a_y - m.b_x / m.a_x);
        let a = m.target_x / m.a_x - b * (m.b_x / m.a_x);
        Some((a, b))
    }
}

#[aoc(day13, part1)]
fn one(machines: &Input) -> Output {
    let mut tokens = 0;
    for machine in machines {
        let s = solve_linear(machine);
        // dbg!(&machine, &s);
        if let Some((a, b)) = s {
            if a > Num::ZERO && b > Num::ZERO && a.is_integer() && b.is_integer() {
                // eprintln!("Found solution {a} {b}");
                tokens += 3 * (*a.numer() as usize) + (*b.numer() as usize);
            }
        }
    }
    tokens
}

#[aoc(day13, part2)]
fn two(machines: &Input) -> Output {
    let offset: Num = Num::from(10000000000000);
    let mut tokens = 0;
    for machine in machines {
        let mut machine = machine.clone();
        machine.target_x += offset;
        machine.target_y += offset;
        let s = solve_linear(&machine);
        // dbg!(&machine, &s);
        if let Some((a, b)) = s {
            if a > Num::ZERO && b > Num::ZERO && a.is_integer() && b.is_integer() {
                // eprintln!("Found solution {a} {b}");
                tokens += 3 * (*a.numer() as usize) + (*b.numer() as usize);
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
