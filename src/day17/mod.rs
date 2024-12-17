use aoc_runner_derive::{aoc, aoc_generator};
use std::arch::x86_64::*;

use crate::util::{parse_digit, parse_initial_digits};

#[derive(Debug, Clone)]
struct Tritron2417 {
    instruction_pointer: usize,
    a: u64,
    b: u64,
    c: u64,
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
        instruction_pointer: 0,
        a: a as u64,
        b: b as u64,
        c: c as u64,
        rom: program,
        output: Vec::with_capacity(input.len()),
    }
}

#[derive(Debug, Clone, Copy)]
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

impl Tritron2417 {
    #[inline(always)]
    fn eval_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!("illegal combo operand"),
        }
    }

    fn cycle(&mut self) {
        if self.instruction_pointer < self.rom.len() {
            let opcode: Opcode = Opcode::from_u8(self.rom[self.instruction_pointer]);
            let operand = self.rom[self.instruction_pointer + 1];

            match opcode {
                Opcode::ADV => {
                    let operand = self.eval_combo_operand(operand);
                    self.a >>= operand;
                }
                Opcode::BXL => {
                    self.b ^= operand as u64;
                }
                Opcode::BST => {
                    let operand = self.eval_combo_operand(operand);
                    self.b = operand % 8;
                }
                Opcode::JNZ => {
                    if self.a != 0 {
                        self.instruction_pointer = operand as usize;
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
            self.instruction_pointer += 2;
        }
    }

    fn run_until_halt(&mut self) {
        while self.instruction_pointer < self.rom.len() {
            self.cycle();
        }
    }

    fn reset(&mut self, a: u64, b: u64, c: u64) {
        self.instruction_pointer = 0;
        self.a = a;
        self.b = b;
        self.c = c;
        self.output.clear();
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

fn search_start_value(tritron: &mut Tritron2417x4, from_pos: usize, a_base: u64) -> Option<u64> {
    for a in (a_base..(a_base + 8)).step_by(4) {
        tritron.reset([a, a + 1, a + 2, a + 3], [0; 4], [0; 4]);
        // dbg!(&tritron.a);
        tritron.run_until_halt();
        // dbg!(&tritron.output);

        let mut correct = [false; 4];
        for i in 0..4 {
            if tritron.output[i] == tritron.rom[from_pos..] {
                correct[i] = true;
            }
        }
        for i in 0..4 {
            if correct[i] {
                let a = a + i as u64;
                // println!(
                //     "SUCC({from_pos}) {a:b} gives {:?}, descending",
                //     &tritron.output[i]
                // );
                if from_pos == 0 {
                    return Some(a);
                }
                if let Some(res) = search_start_value(tritron, from_pos - 1, a << 3) {
                    return Some(res);
                }
            }
        }
    }
    // println!(
    //     "FAIL({from_pos}) {a_base:b} not viable for {:?}, backtracking",
    //     &tritron.rom[from_pos..]
    // );
    None
}

#[aoc(day17, part2)]
fn two(input: &Tritron2417) -> u64 {
    let mut tritron = Tritron2417x4::splat(input);
    // Assumptions: program ends with ADV 3, OUT _, JNZ 0 and has no other JNZs, OUTs, or ADVs.
    search_start_value(&mut tritron, input.rom.len() - 1, 0).expect("solution not found")
}

pub fn part1(puzzle: &str) -> String {
    one(&parse(puzzle.as_bytes()))
}

pub fn part2(puzzle: &str) -> u64 {
    two(&parse(puzzle.as_bytes()))
}

#[derive(Debug, Clone)]
struct Tritron2417x4 {
    instruction_pointer: __m256i,
    a: __m256i,
    b: __m256i,
    c: __m256i,
    output: [Vec<u8>; 4],
    rom: Vec<u8>,
}

impl Tritron2417x4 {
    fn splat(template: &Tritron2417) -> Self {
        unsafe {
            Tritron2417x4 {
                instruction_pointer: _mm256_set1_epi64x(u64_as_i64(
                    template.instruction_pointer as u64,
                )),
                a: _mm256_set1_epi64x(u64_as_i64(template.a)),
                b: _mm256_set1_epi64x(u64_as_i64(template.b)),
                c: _mm256_set1_epi64x(u64_as_i64(template.c)),
                output: [
                    template.output.clone(),
                    template.output.clone(),
                    template.output.clone(),
                    template.output.clone(),
                ],
                rom: template.rom.clone(),
            }
        }
    }

    fn reset(&mut self, a: [u64; 4], b: [u64; 4], c: [u64; 4]) {
        unsafe {
            self.instruction_pointer = _mm256_setzero_si256();
            self.a = _mm256_set_epi64x(
                u64_as_i64(a[3]),
                u64_as_i64(a[2]),
                u64_as_i64(a[1]),
                u64_as_i64(a[0]),
            );
            self.b = _mm256_set_epi64x(
                u64_as_i64(b[3]),
                u64_as_i64(b[2]),
                u64_as_i64(b[1]),
                u64_as_i64(b[0]),
            );
            self.c = _mm256_set_epi64x(
                u64_as_i64(c[3]),
                u64_as_i64(c[2]),
                u64_as_i64(c[1]),
                u64_as_i64(c[0]),
            );
        }
        self.output[0].clear();
        self.output[1].clear();
        self.output[2].clear();
        self.output[3].clear();
    }

    fn eval_combo_operand(&self, operand: u8) -> __m256i {
        match operand {
            0..=3 => unsafe { _mm256_set1_epi64x(u64_as_i64(operand as u64)) },
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!("illegal combo operand"),
        }
    }

    fn cycle(&mut self) {
        let ptr = as_array(&self.instruction_pointer)
            .iter()
            .find(|ip| i64_as_u64(**ip) < self.rom.len() as u64);
        if let Some(&ptr) = ptr {
            let active = unsafe {
                _mm256_cmpeq_epi64(
                    self.instruction_pointer,
                    _mm256_set1_epi64x(u64_as_i64(ptr as u64)),
                )
            };
            let opcode: Opcode = Opcode::from_u8(self.rom[ptr as usize]);
            let operand = self.rom[ptr as usize + 1];
            // dbg!(
            //     &self.instruction_pointer,
            //     &opcode,
            //     &operand,
            //     &self.a,
            //     &active
            // );

            match opcode {
                Opcode::ADV => unsafe {
                    let operand = self.eval_combo_operand(operand);
                    let res = _mm256_srlv_epi64(self.a, operand);
                    self.a = _mm256_blendv_epi8(self.a, res, active);
                },
                Opcode::BXL => unsafe {
                    let operand = _mm256_set1_epi64x(operand as i64);
                    let res = _mm256_xor_si256(self.b, operand);
                    self.b = _mm256_blendv_epi8(self.b, res, active);
                },
                Opcode::BST => unsafe {
                    let operand = self.eval_combo_operand(operand);
                    let mask = _mm256_set1_epi64x(7);
                    let res = _mm256_and_si256(operand, mask);
                    self.b = _mm256_blendv_epi8(self.b, res, active);
                },
                Opcode::JNZ => unsafe {
                    let a_is_zero = _mm256_cmpeq_epi64(self.a, _mm256_setzero_si256());
                    let next_ip = _mm256_add_epi64(self.instruction_pointer, _mm256_set1_epi64x(2));
                    let operand = _mm256_set1_epi64x(u64_as_i64(operand as u64));

                    let res = _mm256_blendv_epi8(operand, next_ip, a_is_zero);
                    self.instruction_pointer =
                        _mm256_blendv_epi8(self.instruction_pointer, res, active);
                    return;
                },
                Opcode::BXC => unsafe {
                    let res = _mm256_xor_si256(self.b, self.c);
                    self.b = _mm256_blendv_epi8(self.b, res, active);
                },
                Opcode::OUT => unsafe {
                    let operand = self.eval_combo_operand(operand);
                    let mask = _mm256_set1_epi64x(7);
                    let operand_mod_8 = _mm256_and_si256(operand, mask);
                    let res = as_array(&operand_mod_8);
                    let active = as_array(&active);
                    for i in 0..4 {
                        if active[i] != 0 {
                            self.output[i].push((res[i] % 8) as u8);
                        }
                    }
                },
                Opcode::BDV => unsafe {
                    let operand = self.eval_combo_operand(operand);
                    let res = _mm256_srlv_epi64(self.a, operand);
                    self.b = _mm256_blendv_epi8(self.b, res, active);
                },
                Opcode::CDV => unsafe {
                    let operand = self.eval_combo_operand(operand);
                    let res = _mm256_srlv_epi64(self.a, operand);
                    self.c = _mm256_blendv_epi8(self.c, res, active);
                },
            }

            self.instruction_pointer =
                unsafe { _mm256_add_epi64(self.instruction_pointer, _mm256_set1_epi64x(2)) };
        }
    }

    fn run_until_halt(&mut self) {
        while as_array(&self.instruction_pointer) != &[u64_as_i64(self.rom.len() as u64); 4] {
            self.cycle();
        }
    }
}

#[cfg(test)]
mod wide_tests {
    use super::*;

    #[test]
    fn test_adv_literal() {
        let base_tritron = Tritron2417 {
            rom: vec![Opcode::ADV as u8, 1],
            instruction_pointer: 0,
            a: 100,
            b: 0,
            c: 0,
            output: vec![],
        };

        let mut tritron = Tritron2417x4::splat(&base_tritron);
        tritron.reset([10, 20, 50, 101], [0; 4], [0; 4]);
        tritron.cycle();
        assert_eq!(as_array(&tritron.a), &[5, 10, 25, 50]);
    }

    #[test]
    fn test_adv() {
        for i in 0..7 {
            let mut base_tritron = Tritron2417 {
                rom: vec![Opcode::ADV as u8, i],
                instruction_pointer: 0,
                a: 20,
                b: 0,
                c: 6,
                output: vec![],
            };

            let mut tritron = Tritron2417x4::splat(&base_tritron);

            tritron.cycle();
            base_tritron.cycle();

            assert_eq!(as_array(&tritron.a), &[u64_as_i64(base_tritron.a); 4]);
            assert_eq!(as_array(&tritron.b), &[u64_as_i64(base_tritron.b); 4]);
            assert_eq!(as_array(&tritron.c), &[u64_as_i64(base_tritron.c); 4]);
        }
    }

    #[test]
    fn test_bxl() {
        for i in 0..8 {
            let mut base_tritron = Tritron2417 {
                rom: vec![Opcode::BXL as u8, i],
                instruction_pointer: 0,
                a: 0,
                b: 0b101010,
                c: 0,
                output: vec![],
            };

            let mut tritron = Tritron2417x4::splat(&base_tritron);

            tritron.cycle();
            base_tritron.cycle();

            assert_eq!(as_array(&tritron.a), &[u64_as_i64(base_tritron.a); 4]);
            assert_eq!(as_array(&tritron.b), &[u64_as_i64(base_tritron.b); 4]);
            assert_eq!(as_array(&tritron.c), &[u64_as_i64(base_tritron.c); 4]);
        }
    }

    #[test]
    fn test_bst() {
        for i in 0..7 {
            let mut base_tritron = Tritron2417 {
                rom: vec![Opcode::BST as u8, i],
                instruction_pointer: 0,
                a: 0b110110,
                b: 0b101010,
                c: 0,
                output: vec![],
            };
            let mut tritron = Tritron2417x4::splat(&base_tritron);
            tritron.cycle();
            base_tritron.cycle();
            assert_eq!(as_array(&tritron.a), &[u64_as_i64(base_tritron.a); 4]);
            assert_eq!(as_array(&tritron.b), &[u64_as_i64(base_tritron.b); 4]);
            assert_eq!(as_array(&tritron.c), &[u64_as_i64(base_tritron.c); 4]);
        }
    }

    #[test]
    fn test_bxc() {
        for i in 0..7 {
            let mut base_tritron = Tritron2417 {
                rom: vec![Opcode::BXC as u8, i],
                instruction_pointer: 0,
                a: 0b110110,
                b: 0b101010,
                c: 0,
                output: vec![],
            };
            let mut tritron = Tritron2417x4::splat(&base_tritron);
            tritron.cycle();
            base_tritron.cycle();
            assert_eq!(as_array(&tritron.a), &[u64_as_i64(base_tritron.a); 4]);
            assert_eq!(as_array(&tritron.b), &[u64_as_i64(base_tritron.b); 4]);
            assert_eq!(as_array(&tritron.c), &[u64_as_i64(base_tritron.c); 4]);
        }
    }

    #[test]
    fn test_bdv() {
        for i in 0..7 {
            let mut base_tritron = Tritron2417 {
                rom: vec![Opcode::BDV as u8, i],
                instruction_pointer: 0,
                a: 0b110110,
                b: 0b101010,
                c: 0,
                output: vec![],
            };
            let mut tritron = Tritron2417x4::splat(&base_tritron);
            tritron.cycle();
            base_tritron.cycle();
            assert_eq!(as_array(&tritron.a), &[u64_as_i64(base_tritron.a); 4]);
            assert_eq!(as_array(&tritron.b), &[u64_as_i64(base_tritron.b); 4]);
            assert_eq!(as_array(&tritron.c), &[u64_as_i64(base_tritron.c); 4]);
        }
    }

    #[test]
    fn test_cdv() {
        for i in 0..7 {
            let mut base_tritron = Tritron2417 {
                rom: vec![Opcode::CDV as u8, i],
                instruction_pointer: 0,
                a: 0b110110,
                b: 0b101010,
                c: 0,
                output: vec![],
            };
            let mut tritron = Tritron2417x4::splat(&base_tritron);
            tritron.cycle();
            base_tritron.cycle();
            assert_eq!(as_array(&tritron.a), &[u64_as_i64(base_tritron.a); 4]);
            assert_eq!(as_array(&tritron.b), &[u64_as_i64(base_tritron.b); 4]);
            assert_eq!(as_array(&tritron.c), &[u64_as_i64(base_tritron.c); 4]);
        }
    }
}

#[inline(always)]
fn as_array(x: &__m256i) -> &[i64; 4] {
    unsafe { std::mem::transmute(x) }
}

#[inline(always)]
fn u64_as_i64(x: u64) -> i64 {
    unsafe { std::mem::transmute(x) }
}

#[inline(always)]
fn i64_as_u64(x: i64) -> u64 {
    unsafe { std::mem::transmute(x) }
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
