use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
enum ReaderState {
    Init,
    ReadM,
    ReadMU,
    ReadMUL,
    FirstDigit(String),
    SecondDigit(u64, String),
    ReadD,
    ReadDO,
    ReadDOLparen,
    ReadDON,
    ReadDONApo,
    ReadDONT,
    ReadDONTLparen,
}

#[derive(Clone, Debug)]

struct MultiplicationReader {
    pairs: Vec<(bool, u64, u64)>,
    state: ReaderState,
    doing: bool,
}

impl MultiplicationReader {
    fn new() -> Self {
        Self {
            pairs: Vec::new(),
            state: ReaderState::Init,
            doing: true,
        }
    }

    fn read(&mut self, ch: char) {
        match (&mut self.state, ch) {
            (ReaderState::Init, 'm') => self.state = ReaderState::ReadM,
            (ReaderState::ReadM, 'u') => self.state = ReaderState::ReadMU,
            (ReaderState::ReadMU, 'l') => self.state = ReaderState::ReadMUL,
            (ReaderState::ReadMUL, '(') => {
                self.state = ReaderState::FirstDigit(String::with_capacity(3))
            }
            (ReaderState::FirstDigit(first), ',') => {
                self.state =
                    ReaderState::SecondDigit(first.parse().unwrap(), String::with_capacity(3))
            }
            (ReaderState::FirstDigit(ref mut first), '0'..='9') => {
                if first.len() < 3 {
                    first.push(ch)
                } else {
                    self.state = ReaderState::Init
                }
            }
            (ReaderState::SecondDigit(_, ref mut second), '0'..='9') => {
                if second.len() < 3 {
                    second.push(ch)
                } else {
                    self.state = ReaderState::Init
                }
            }
            (ReaderState::SecondDigit(first, second), ')') => {
                self.pairs
                    .push((self.doing, *first, second.parse().unwrap()));
                self.state = ReaderState::Init;
            }
            (ReaderState::Init, 'd') => self.state = ReaderState::ReadD,
            (ReaderState::ReadD, 'o') => self.state = ReaderState::ReadDO,
            (ReaderState::ReadDO, '(') => self.state = ReaderState::ReadDOLparen,
            (ReaderState::ReadDOLparen, ')') => {
                self.doing = true;
                self.state = ReaderState::Init;
            }
            (ReaderState::ReadDO, 'n') => self.state = ReaderState::ReadDON,
            (ReaderState::ReadDON, '\'') => self.state = ReaderState::ReadDONApo,
            (ReaderState::ReadDONApo, 't') => self.state = ReaderState::ReadDONT,
            (ReaderState::ReadDONT, '(') => self.state = ReaderState::ReadDONTLparen,
            (ReaderState::ReadDONTLparen, ')') => {
                self.doing = false;
                self.state = ReaderState::Init;
            }
            _ => {
                self.state = ReaderState::Init;
            }
        };
    }
}

type Input = Vec<(bool, u64, u64)>;
type Output = u64;

#[aoc_generator(day3)]
fn parse(puzzle: &str) -> Input {
    let mut reader = MultiplicationReader::new();
    puzzle.chars().for_each(|c| reader.read(c));
    reader.pairs
}

#[aoc(day3, part1)]
fn part_one(input: &Input) -> Output {
    input.iter().map(|(_, a, b)| a * b).sum()
}

pub fn part1(puzzle: &str) -> Output {
    part_one(&parse(puzzle))
}

#[aoc(day3, part2)]
fn part_two(input: &Input) -> Output {
    input
        .iter()
        .map(|(doing, a, b)| if *doing { a * b } else { 0 })
        .sum()
}

pub fn part2(puzzle: &str) -> Output {
    part_two(&parse(puzzle))
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, 161);
    }

    #[test]
    fn example2() {
        let res = part2(include_str!("test_2.txt"));
        assert_eq!(res, 48);
    }
}
