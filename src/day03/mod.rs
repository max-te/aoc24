use crate::Solutions;
use lib_aoc::prelude::*;

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

    fn read(&mut self, char: char) {
        match (&mut self.state, char) {
            (ReaderState::Init, 'm') => self.state = ReaderState::ReadM,
            (ReaderState::ReadM, 'u') => self.state = ReaderState::ReadMU,
            (ReaderState::ReadMU, 'l') => self.state = ReaderState::ReadMUL,
            (ReaderState::ReadMUL, '(') => self.state = ReaderState::FirstDigit(String::new()),
            (ReaderState::FirstDigit(first), ',') => {
                self.state = ReaderState::SecondDigit(first.parse().unwrap(), String::new())
            }
            (ReaderState::FirstDigit(ref mut first), '0'..='9') => first.push(char),
            (ReaderState::SecondDigit(_, ref mut second), '0'..='9') => second.push(char),
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

impl Solution<DAY_03> for Solutions {
    type Input<'i> = Vec<(bool, u64, u64)>;
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let mut reader = MultiplicationReader::new();
        puzzle.chars().for_each(|c| reader.read(c));
        reader.pairs
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        input.iter().map(|(_, a, b)| a * b).sum()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        input
            .iter()
            .map(|(doing, a, b)| if *doing { a * b } else { 0 })
            .sum()
    }
}

impl Test<DAY_03> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 161,
            PART_TWO => 48,
        }
    }
}

derive_tests!(Solutions, DAY_03);
