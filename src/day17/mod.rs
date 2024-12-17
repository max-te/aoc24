use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::{parse_digit, parse_initial_digits};

#[derive(Debug, Clone)]
struct Tritron2417 {
    instruction_pointer: Option<usize>,
    a: usize,
    b: usize,
    c: usize,
    rom: Vec<u8>,
    output: Vec<u8>,
}

#[aoc_generator(day17)]
fn parse(input: &[u8]) -> Tritron2417 {
    let input = &input[const { "Register A: ".len() }..];
    let (a, num_len) = parse_initial_digits(input);
    let input = &input[num_len + const { "\nRegister B: ".len() }..];
    let (b, num_len) = parse_initial_digits(input);
    let input = &input[num_len + const { "\nRegister C: ".len() }..];
    let (c, num_len) = parse_initial_digits(input);
    let input = &input[num_len + const { "\n\nProgram: ".len() }..];
    let mut program = Vec::with_capacity(input.len() / 2);
    for i in (0..input.len()).step_by(2) {
        program.push(parse_digit(&input[i]));
    }

    Tritron2417 {
        instruction_pointer: Some(0),
        a: a as usize,
        b: b as usize,
        c: c as usize,
        rom: program,
        output: Vec::with_capacity(input.len()),
    }
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
    fn from_u8(num: u8) -> Option<Self> {
        match num {
            0 => Some(Self::ADV),
            1 => Some(Self::BXL),
            2 => Some(Self::BST),
            3 => Some(Self::JNZ),
            4 => Some(Self::BXC),
            5 => Some(Self::OUT),
            6 => Some(Self::BDV),
            7 => Some(Self::CDV),
            _ => None,
        }
    }
}

impl Tritron2417 {
    fn eval_combo_operand(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!("illegal combo operand"),
        }
    }

    fn cycle(&mut self) {
        if let Some(ptr) = self.instruction_pointer {
            let opcode = Opcode::from_u8(self.rom[ptr]).unwrap();
            let operand = self.rom[ptr + 1];

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
                        self.instruction_pointer = Some(operand as usize);
                        return;
                    }
                }
                Opcode::BXC => {
                    self.b ^= self.c;
                }
                Opcode::OUT => {
                    let operand = self.eval_combo_operand(operand);
                    self.output.push((operand % 8) as u8);
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
            if ptr + 2 == self.rom.len() {
                self.instruction_pointer = None
            } else {
                self.instruction_pointer = Some(ptr + 2);
            }
        }
    }

    fn run_until_halt(&mut self) {
        while self.instruction_pointer.is_some() {
            self.cycle();
        }
    }
}

#[aoc(day17, part1)]
fn one(input: &Tritron2417) -> String {
    let mut tritron = input.clone();
    tritron.run_until_halt();
    let mut s = tritron.output.iter().fold(
        String::with_capacity(tritron.output.len() * 2),
        |mut s, n| {
            s.push(char::from_digit(*n as u32, 10).unwrap());
            s.push(',');
            s
        },
    );
    s.pop();
    s
}

fn search_start_value(input: &Tritron2417, from_pos: usize, a_base: usize) -> Option<usize> {
    for e in a_base..(a_base + 8) {
        let mut tritron = input.clone();
        tritron.a = e;
        tritron.run_until_halt();
        if tritron.output == &input.rom[from_pos..] {
            // println!("SUCC({from_pos}) {e:b} gives {:?}, descending", &tritron.output);
            if from_pos == 0 {
                return Some(e);
            }
            if let Some(res) = search_start_value(input, from_pos - 1, e << 3) {
                return Some(res);
            }
        }
    }
    // println!("FAIL({from_pos}) {a_base:b} not viable for {:?}, backtracking", &input.rom[from_pos..]);
    None
}

#[aoc(day17, part2)]
fn two(input: &Tritron2417) -> usize {
    // Assumptions: program ends with ADV 3, OUT _, JNZ 0 and has no other JNZs, OUTs, or ADVs.
    let res = search_start_value(input, input.rom.len() - 1, 0).expect("solution not found");
    let mut tritron = input.clone();
    tritron.a = res;
    tritron.run_until_halt();
    res
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
