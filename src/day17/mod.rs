use aoc_runner_derive::{aoc, aoc_generator};
use smallvec::SmallVec;

use crate::util::{parse_digit, parse_initial_digits};

#[derive(Debug, Clone)]
struct Tritron2417<'p> {
    instruction_pointer: usize,
    a: usize,
    b: usize,
    c: usize,
    rom: &'p [u8],
}

type Input = (usize, usize, usize, SmallVec<[u8; 16]>);

#[aoc_generator(day17)]
fn parse(input: &[u8]) -> Input {
    let input = &input[const { "Register A: ".len() }..];
    let (a, num_len) = parse_initial_digits(input);
    let input = &input[num_len + const { "\nRegister B: ".len() }..];
    let (b, num_len) = parse_initial_digits(input);
    let input = &input[num_len + const { "\nRegister C: ".len() }..];
    let (c, num_len) = parse_initial_digits(input);
    let input = &input[num_len + const { "\n\nProgram: ".len() }..];
    let mut program = SmallVec::new();
    for i in (0..input.len()).step_by(2) {
        program.push(parse_digit(&input[i]));
    }

    (a as usize, b as usize, c as usize, program)
}

#[repr(u8)]
enum Opcode {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}

impl Opcode {
    fn from_u8(num: u8) -> Self {
        match num {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => unreachable!(),
        }
    }
}

impl<'p> Tritron2417<'p> {
    #[inline(always)]
    fn eval_combo_operand(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!("illegal combo operand"),
        }
    }

    fn cycle(&mut self) -> Option<u8> {
        if self.instruction_pointer < self.rom.len() {
            let opcode: Opcode = Opcode::from_u8(self.rom[self.instruction_pointer]);
            let operand = self.rom[self.instruction_pointer + 1];

            match opcode {
                Opcode::ADV => {
                    let operand = self.eval_combo_operand(operand);
                    self.a >>= operand;
                }
                Opcode::BXL => {
                    self.b ^= operand as usize;
                }
                Opcode::BST => {
                    let operand = self.eval_combo_operand(operand);
                    self.b = operand % 8;
                }
                Opcode::JNZ => {
                    if self.a != 0 {
                        self.instruction_pointer = operand as usize;
                        return None;
                    }
                }
                Opcode::BXC => {
                    self.b ^= self.c;
                }
                Opcode::OUT => {
                    let operand = self.eval_combo_operand(operand);
                    self.instruction_pointer += 2;
                    return Some(operand as u8 % 8);
                }
                Opcode::BDV => {
                    let operand = self.eval_combo_operand(operand);
                    self.b = self.a >> operand;
                }
                Opcode::CDV => {
                    let operand = self.eval_combo_operand(operand);
                    self.c = self.a >> operand;
                }
            }
            self.instruction_pointer += 2;
        }
        None
    }

    fn run_until_halt(&mut self) {
        while self.instruction_pointer < self.rom.len() {
            self.cycle();
        }
    }

    fn run_until_next_output(&mut self) -> Option<u8> {
        while self.instruction_pointer < self.rom.len() {
            if let Some(out) = self.cycle() {
                return Some(out);
            }
        }
        None
    }

    fn reset(&mut self, a: usize, b: usize, c: usize) {
        self.instruction_pointer = 0;
        self.a = a;
        self.b = b;
        self.c = c;
    }
}

#[aoc(day17, part1)]
fn one((a, b, c, program): &Input) -> String {
    let mut res = String::with_capacity(program.len() * 2);
    let mut tritron = Tritron2417 {
        rom: program,
        instruction_pointer: 0,
        a: *a,
        b: *b,
        c: *c,
    };
    while let Some(out) = tritron.run_until_next_output() {
        res.push(char::from_u32((out + b'0') as u32).unwrap());
        res.push(',');
    }
    res.pop();
    res
}

fn search_start_value(tritron: &mut Tritron2417, from_pos: usize, a_base: usize) -> Option<usize> {
    'a: for a in a_base..(a_base + 8) {
        tritron.reset(a, 0, 0);
        for i in from_pos..tritron.rom.len() {
            let out = tritron.run_until_next_output();
            if out != Some(tritron.rom[i]) {
                continue 'a;
            }
        }
        if tritron.run_until_next_output().is_none() {
            // println!("SUCC({from_pos}) {e:b} gives {:?}, descending", &tritron.output);
            if from_pos == 0 {
                return Some(a);
            }
            if let Some(res) = search_start_value(tritron, from_pos - 1, a << 3) {
                return Some(res);
            }
        }
    }
    // println!("FAIL({from_pos}) {a_base:b} not viable for {:?}, backtracking", &input.rom[from_pos..]);
    None
}

#[aoc(day17, part2)]
fn two((_, _, _, program): &Input) -> usize {
    let mut tritron = Tritron2417 {
        rom: program,
        instruction_pointer: 0,
        a: 0,
        b: 0,
        c: 0,
    };
    // Assumptions: program ends with ADV 3, OUT _, JNZ 0 and has no other JNZs, OUTs, or ADVs.
    // B & C always start at 0.
    search_start_value(&mut tritron, program.len() - 1, 0).expect("solution not found")
}

pub fn part1(puzzle: &str) -> String {
    one(&parse(puzzle.as_bytes()))
}

pub fn part2(puzzle: &str) -> usize {
    two(&parse(puzzle.as_bytes()))
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn example2() {
        let res = part2(include_str!("test2.txt"));
        assert_eq!(res, 117440);
    }
}
