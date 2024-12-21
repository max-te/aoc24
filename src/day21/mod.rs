use std::arch::x86_64::*;

use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use rustc_hash::FxHashMap;

type Input = [[u8; 4]; 5];

pub fn part1(puzzle: &str) -> u64 {
    one_code_lut_simd(puzzle)
}

pub fn part2(puzzle: &str) -> u64 {
    two_code_lut_simd(puzzle)
}

#[aoc_generator(day21, part1, naive)]
#[aoc_generator(day21, part1, recursive)]
#[aoc_generator(day21, part2, recursive)]
#[aoc_generator(day21, part1, lut)]
#[aoc_generator(day21, part2, lut)]
pub fn parse(input: &str) -> Input {
    let input = input.as_bytes();
    let mut result = [[0; 4]; 5];
    for i in 0..5 {
        result[i].copy_from_slice(&input[i * 5..i * 5 + 4]);
    }
    result
}

#[aoc(day21, part1, naive)]
pub fn one(input: &Input) -> usize {
    let mut res = 0;
    for code in input {
        let move_count = input_code(*code).len();
        let value = (code[0] - b'0') as usize * 100
            + (code[1] - b'0') as usize * 10
            + (code[2] - b'0') as usize;
        res += value * move_count;
    }
    res
}

#[aoc(day21, part1, recursive)]
pub fn one_recursive(input: &Input) -> usize {
    let mut res = 0;
    let mut memo = FxHashMap::default();
    for code in input {
        let move_count = input_code_recursive(*code, 2, &mut memo);
        let value = (code[0] - b'0') as usize * 100
            + (code[1] - b'0') as usize * 10
            + (code[2] - b'0') as usize;
        res += value * move_count;
    }
    res
}

#[aoc(day21, part2, recursive)]
pub fn two(input: &Input) -> usize {
    let mut res = 0;
    let mut memo = FxHashMap::default();
    for code in input {
        let move_count = input_code_recursive(*code, 25, &mut memo);
        let value = (code[0] - b'0') as usize * 100
            + (code[1] - b'0') as usize * 10
            + (code[2] - b'0') as usize;
        res += value * move_count;
    }
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DPadPress {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl DPadPress {
    const fn values() -> &'static [DPadPress] {
        use DPadPress::*;
        &[Up, Down, Left, Right, Activate]
    }
}

impl std::fmt::Display for DPadPress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DPadPress::Up => write!(f, "^"),
            DPadPress::Down => write!(f, "v"),
            DPadPress::Left => write!(f, "<"),
            DPadPress::Right => write!(f, ">"),
            DPadPress::Activate => write!(f, "A"),
        }
    }
}

#[inline]
const fn numpad_position(key: u8) -> (usize, usize) {
    match key {
        b'7' => (0, 0),
        b'8' => (1, 0),
        b'9' => (2, 0),

        b'4' => (0, 1),
        b'5' => (1, 1),
        b'6' => (2, 1),

        b'1' => (0, 2),
        b'2' => (1, 2),
        b'3' => (2, 2),

        b'0' => (1, 3),
        b'A' => (2, 3),

        _ => unreachable!(),
    }
}

#[inline]
fn numpad_one_move(from: u8, to: u8) -> ArrayVec<DPadPress, 6> {
    let mut moves = ArrayVec::new();
    let pos = numpad_position(from);
    let target_pos = numpad_position(to);
    let v_moves: &[DPadPress] = if target_pos.1 <= pos.1 {
        match pos.1 - target_pos.1 {
            0 => &[],
            1 => &[DPadPress::Up],
            2 => &[DPadPress::Up, DPadPress::Up],
            3 => &[DPadPress::Up, DPadPress::Up, DPadPress::Up],
            _ => unreachable!(),
        }
    } else {
        match target_pos.1 - pos.1 {
            1 => &[DPadPress::Down],
            2 => &[DPadPress::Down, DPadPress::Down],
            3 => &[DPadPress::Down, DPadPress::Down, DPadPress::Down],
            _ => unreachable!(),
        }
    };

    // We want to get all the left moves out of the way first if at all possible
    // then up/down, then right. Since every left move at stage 1 needs two left moves at stage 2,
    // which we want to be sequential and not interleaved with other moves.

    if target_pos.0 > pos.0 {
        if pos.0 == 0 && target_pos.1 == 3 {
            // First right, so we don't crash
            for _ in 0..target_pos.0 - pos.0 {
                moves.push(DPadPress::Right);
            }
            for v_move in v_moves {
                moves.push(*v_move);
            }
        } else {
            // First vertical, then right
            for v_move in v_moves {
                moves.push(*v_move);
            }
            for _ in 0..target_pos.0 - pos.0 {
                moves.push(DPadPress::Right);
            }
        }
    } else if pos.1 == 3 && target_pos.0 == 0 {
        // First up, so we don't crash
        for v_move in v_moves {
            moves.push(*v_move);
        }
        for _ in 0..pos.0 - target_pos.0 {
            moves.push(DPadPress::Left);
        }
    } else {
        // First left, then vertical
        for _ in 0..pos.0 - target_pos.0 {
            moves.push(DPadPress::Left);
        }
        for v_move in v_moves {
            moves.push(*v_move);
        }
    }
    moves.push(DPadPress::Activate);
    moves
}

fn numpad_moves(code: &[u8; 4]) -> ArrayVec<DPadPress, 24> {
    let mut moves = ArrayVec::new();
    let mut prev_key = b'A';
    for &target_key in code {
        moves.extend(numpad_one_move(prev_key, target_key));
        prev_key = target_key;
    }
    moves
}

#[inline]
const fn dpad_one_move(from: DPadPress, to: DPadPress) -> &'static [DPadPress] {
    use DPadPress::*;
    match (from, to) {
        (Up, Up) => &[Activate],
        (Up, Down) => &[Down, Activate],
        (Up, Left) => &[Down, Left, Activate],
        (Up, Right) => &[Down, Right, Activate],
        (Up, Activate) => &[Right, Activate],
        (Down, Up) => &[Up, Activate],
        (Down, Down) => &[Activate],
        (Down, Left) => &[Left, Activate],
        (Down, Right) => &[Right, Activate],
        (Down, Activate) => &[Up, Right, Activate],
        (Left, Up) => &[Right, Up, Activate],
        (Left, Down) => &[Right, Activate],
        (Left, Left) => &[Activate],
        (Left, Right) => &[Right, Right, Activate],
        (Left, Activate) => &[Right, Right, Up, Activate],
        (Right, Up) => &[Left, Up, Activate],
        (Right, Down) => &[Left, Activate],
        (Right, Left) => &[Left, Left, Activate],
        (Right, Right) => &[Activate],
        (Right, Activate) => &[Up, Activate],
        (Activate, Up) => &[Left, Activate],
        (Activate, Down) => &[Left, Down, Activate],
        (Activate, Left) => &[Down, Left, Left, Activate],
        (Activate, Right) => &[Down, Activate],
        (Activate, Activate) => &[Activate],
    }
}

fn dpad_moves(code: &[DPadPress]) -> Vec<DPadPress> {
    let mut moves = Vec::new();
    let mut prev_key = DPadPress::Activate;
    for target_key in code {
        moves.extend_from_slice(dpad_one_move(prev_key, *target_key));
        prev_key = *target_key;
    }
    moves
}

fn input_code(code: [u8; 4]) -> Vec<DPadPress> {
    #[cfg(debug_assertions)]
    eprintln!("{}", String::from_utf8_lossy(&code));
    let stage1 = numpad_moves(&code);
    #[cfg(debug_assertions)]
    for mov in stage1.iter() {
        print!("{mov}");
    }
    let stage2 = dpad_moves(&stage1);
    #[cfg(debug_assertions)]
    for mov in stage2.iter() {
        print!("{mov}");
    }
    let stage3 = dpad_moves(&stage2);
    #[cfg(debug_assertions)]
    for mov in stage3.iter() {
        print!("{mov}");
    }
    stage3
}

fn dpad_one_move_recursive(
    from: DPadPress,
    to: DPadPress,
    depth: usize,
    memo: &mut FxHashMap<(DPadPress, DPadPress, usize), usize>,
) -> usize {
    let path = dpad_one_move(from, to);
    if depth == 1 {
        path.len()
    } else {
        let key = (from, to, depth);
        if let Some(res) = memo.get(&key) {
            *res
        } else {
            let mut len = 0;
            let mut last_pos = DPadPress::Activate;
            for next in path {
                len += dpad_one_move_recursive(last_pos, *next, depth - 1, memo);
                last_pos = *next;
            }
            memo.insert(key, len);
            len
        }
    }
}

fn input_code_recursive(
    code: [u8; 4],
    dpad_count: usize,
    dpad_memo: &mut FxHashMap<(DPadPress, DPadPress, usize), usize>,
) -> usize {
    let numpad = numpad_moves(&code);
    let mut len = 0;
    let mut last_pos = DPadPress::Activate;
    for next in numpad {
        len += dpad_one_move_recursive(last_pos, next, dpad_count, dpad_memo);
        last_pos = next;
    }
    len
}

#[aoc(day21, part2, lut)]
pub fn two_lut(input: &Input) -> usize {
    let mut res = 0;
    let lut = const { build_dpad_lut(25) };
    for code in input {
        let move_count = input_code_lut(*code, &lut);
        let value = (code[0] - b'0') as usize * 100
            + (code[1] - b'0') as usize * 10
            + (code[2] - b'0') as usize;
        res += value * move_count;
    }
    res
}

#[aoc(day21, part1, lut)]
pub fn one_lut(input: &Input) -> usize {
    let mut res = 0;
    let lut = const { build_dpad_lut(2) };
    for code in input {
        let move_count = input_code_lut(*code, &lut);
        let value = (code[0] - b'0') as usize * 100
            + (code[1] - b'0') as usize * 10
            + (code[2] - b'0') as usize;
        res += value * move_count;
    }
    res
}

const fn dpad_lut_key(from: DPadPress, to: DPadPress) -> usize {
    from as usize * 5 + to as usize
}

macro_rules! const_for {
    ($index:ident in ($from:expr)..($to:expr) => $body:expr) => {{
        let mut $index = $from;
        while $index < $to {
            $body;
            $index += 1;
        }
    }};
    ($index:ident, $x:ident in $iter:expr => $body:expr) => {{
        let mut $index = 0;
        while $index < $iter.len() {
            let $x = $iter[$index];
            $body;
            $index += 1;
        }
    }};
}

const fn build_dpad_lut(depth: usize) -> [usize; 25] {
    let mut lut = [0; 25];
    if depth == 1 {
        const_for!(from_idx, from in DPadPress::values() => {
            const_for!(to_idx, to in DPadPress::values() => {
                let key = dpad_lut_key(from, to);
                let path = dpad_one_move(from, to);
                lut[key] = path.len();
            })
        })
    } else {
        let prev_lut = build_dpad_lut(depth - 1);
        const_for!(from_idx, from in DPadPress::values() => {
            const_for!(to_idx, to in DPadPress::values() => {
                let key = dpad_lut_key(from, to);
                let path = dpad_one_move(from, to);
                let mut last_pos = DPadPress::Activate;
                const_for!(path_ix, next in path => {
                    lut[key] += prev_lut[dpad_lut_key(last_pos, next)];
                    last_pos = next;
                })
            })
        })
    }
    lut
}

fn input_code_lut(code: [u8; 4], dpad_lut: &[usize; 25]) -> usize {
    let numpad = numpad_moves(&code);
    let mut len = 0;
    let mut last_pos = DPadPress::Activate;
    for next in numpad {
        len += dpad_lut[dpad_lut_key(last_pos, next)];
        last_pos = next;
    }
    len
}

const fn numpad_one_move_array(from: u8, to: u8) -> ([Option<DPadPress>; 6], usize) {
    let mut moves = [None; 6];
    let mut len = 0;

    let pos = numpad_position(from);
    let target_pos = numpad_position(to);

    let v_moves: &[DPadPress] = if target_pos.1 <= pos.1 {
        match pos.1 - target_pos.1 {
            0 => &[],
            1 => &[DPadPress::Up],
            2 => &[DPadPress::Up, DPadPress::Up],
            3 => &[DPadPress::Up, DPadPress::Up, DPadPress::Up],
            _ => unreachable!(),
        }
    } else {
        match target_pos.1 - pos.1 {
            1 => &[DPadPress::Down],
            2 => &[DPadPress::Down, DPadPress::Down],
            3 => &[DPadPress::Down, DPadPress::Down, DPadPress::Down],
            _ => unreachable!(),
        }
    };

    if target_pos.0 > pos.0 {
        if pos.0 == 0 && target_pos.1 == 3 {
            // First right, so we don't crash
            const_for!(_x in (0)..(target_pos.0 - pos.0) => {
                moves[len] = Some(DPadPress::Right);
                len += 1;
            });
            const_for!(mov_idx, v_move in v_moves => {
                moves[len] = Some(v_move);
                len += 1;
            });
        } else {
            // First vertical, then right
            const_for!(mov_idx, v_move in v_moves => {
                moves[len] = Some(v_move);
                len += 1;
            });
            const_for!(_x in (0)..(target_pos.0 - pos.0) => {
                moves[len] = Some(DPadPress::Right);
                len += 1;
            });
        }
    } else if pos.1 == 3 && target_pos.0 == 0 {
        // First up, so we don't crash
        const_for!(mov_idx, v_move in v_moves => {
            moves[len] = Some(v_move);
            len += 1;
        });

        const_for!(_x in (0)..(pos.0 - target_pos.0) => {
            moves[len] = Some(DPadPress::Left);
            len += 1;
        });
    } else {
        // First left, then vertical
        const_for!(_x in (0)..(pos.0 - target_pos.0) => {
            moves[len] = Some(DPadPress::Left);
            len += 1;
        });
        const_for!(mov_idx, v_move in v_moves => {
            moves[len] = Some(v_move);
            len += 1;
        });
    }
    moves[len] = Some(DPadPress::Activate);
    len += 1;
    (moves, len)
}

const NUMPAD_KEYS: [u8; 11] = [
    b'7', b'8', b'9', b'4', b'5', b'6', b'1', b'2', b'3', b'0', b'A',
];

#[inline]
const fn numpad_lut_halfidx(key: u8) -> usize {
    match key {
        b'7' => 0,
        b'8' => 1,
        b'9' => 2,
        b'4' => 3,
        b'5' => 4,
        b'6' => 5,
        b'1' => 6,
        b'2' => 7,
        b'3' => 8,
        b'0' => 9,
        b'A' => 10,
        _ => unreachable!(),
    }
}

#[inline]
const fn numpad_lut_idx(from: u8, to: u8) -> usize {
    let from_idx = numpad_lut_halfidx(from);
    let to_idx = numpad_lut_halfidx(to);
    from_idx * 11 + to_idx
}

const fn build_numpad_lut(dpad_depth: usize) -> [usize; 11 * 11] {
    let dpad_lut = build_dpad_lut(dpad_depth);
    let mut numpad_lut = [0; 11 * 11];

    const_for!(from_idx, from in NUMPAD_KEYS => {
        const_for!(to_idx, to in NUMPAD_KEYS => {
            let key = numpad_lut_idx(from, to);
            let (path, path_len) = numpad_one_move_array(from, to);
            let mut last_pos = DPadPress::Activate;
            const_for!(path_idx in (0)..(path_len) => {
                match path[path_idx] {
                    None => unreachable!(),
                    Some(next) => {
                        numpad_lut[key] += dpad_lut[dpad_lut_key(last_pos, next)];
                        last_pos = next;
                    }
                }
            })
        })
    });

    numpad_lut
}

#[inline]
const fn input_code_numpad_lut(code: [u8; 4], numpad_lut: &[usize; 11 * 11]) -> usize {
    let mut prev_key = b'A';
    let mut res = 0;
    const_for!(c_idx, c in code => {
        res += numpad_lut[numpad_lut_idx(prev_key, c)];
        prev_key = c;
    });
    res
}

#[inline]
#[aoc(day21, part2, num_lut)]
pub fn two_num_lut(input: &str) -> usize {
    let input = input.as_bytes();
    let mut res = 0;
    let lut = const { build_numpad_lut(25) };
    for i in 0..5 {
        let code = [
            input[i * 5],
            input[i * 5 + 1],
            input[i * 5 + 2],
            input[i * 5 + 3],
        ];
        let move_count = input_code_numpad_lut(code, &lut);
        let value = code[0] as usize * 100 + code[1] as usize * 10 + code[2] as usize
            - const { b'0' as usize * 111 };
        res += value * move_count;
    }
    res
}

#[inline]
#[aoc(day21, part1, num_lut)]
pub fn one_num_lut(input: &str) -> usize {
    let input = input.as_bytes();
    let mut res = 0;
    let lut = const { build_numpad_lut(2) };
    for i in 0..5 {
        let code = [
            input[i * 5],
            input[i * 5 + 1],
            input[i * 5 + 2],
            input[i * 5 + 3],
        ];
        let move_count = input_code_numpad_lut(code, &lut);
        let value = code[0] as usize * 100 + code[1] as usize * 10 + code[2] as usize
            - const { b'0' as usize * 111 };
        res += value * move_count;
    }
    res
}

#[inline]
const fn code_lut_idx(code: [u8; 3]) -> usize {
    // ASCII digits are 011_0000 to 011_1001
    ((code[1] as usize & 0b1111) << 8) ^ ((code[2] as usize) << 4) ^ (code[0] as usize) ^ 0b0011
}

const fn build_code_lut(dpad_depth: usize) -> [u64; 4096] {
    let mut code_lut = [0; 4096];
    let numpad_lut = build_numpad_lut(dpad_depth);
    const_for!(code_num in (0)..(1000) => {
        let code = [
            (code_num / 100) as u8 + b'0',
            ((code_num / 10) % 10) as u8 + b'0',
            (code_num % 10) as u8 + b'0',
            b'A',
        ];
        let move_count = input_code_numpad_lut(code, &numpad_lut);
        code_lut[code_lut_idx([code[0], code[1], code[2]])] = (move_count * code_num) as u64;

    });
    code_lut
}

#[inline]
#[aoc(day21, part1, code_lut)]
fn one_code_lut(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut res: u64 = 0;
    let lut = const { build_code_lut(2) };
    for i in 0..5 {
        let code = unsafe {
            [
                *input.get_unchecked(i * 5),
                *input.get_unchecked(i * 5 + 1),
                *input.get_unchecked(i * 5 + 2),
            ]
        };
        let code_lut_idx = code_lut_idx(code);
        let code_score = unsafe { *lut.get_unchecked(code_lut_idx) };
        res = unsafe { res.unchecked_add(code_score) };
    }
    res
}

#[inline]
#[aoc(day21, part2, code_lut)]
fn two_code_lut(input: &str) -> usize {
    let input = input.as_bytes();
    let mut res: u64 = 0;
    let lut = const { build_code_lut(25) };
    for i in 0..5 {
        let code = unsafe {
            [
                *input.get_unchecked(i * 5),
                *input.get_unchecked(i * 5 + 1),
                *input.get_unchecked(i * 5 + 2),
            ]
        };
        res = unsafe { res.unchecked_add(*lut.get_unchecked(code_lut_idx(code))) };
    }
    res as usize
}

#[inline]
fn code_lut_simd<const DEPTH: usize>(input: &str) -> u64 {
    let input = input.as_bytes();
    let lut = const { build_code_lut(DEPTH) }.as_ptr() as *const i64;

    unsafe fn _mm256_hadd_epi64(a: __m256i) -> __m256i {
        let shuf1 = _mm256_permute4x64_epi64::<0b00_01_10_11>(a);
        let a = _mm256_add_epi64(a, shuf1);
        let shuf2 = _mm256_permute4x64_epi64::<0b01_00_00_01>(a);
        _mm256_add_epi64(a, shuf2)
    }

    unsafe {
        let input_offsets = _mm256_set_epi32(0, 5, 10, 15, 20, 20, 20, 20);
        let inputs = _mm256_i32gather_epi32::<1>(input.as_ptr() as *const i32, input_offsets);
        // 0011 XXXX 0011 YYYY 0011 ZZZZ
        let sh_inputs = _mm256_srli_epi32(inputs, 12);
        // >>             0011 XXXX 0011
        let input_indices = _mm256_xor_si256(inputs, sh_inputs);
        let input_indices = _mm256_and_si256(input_indices, _mm256_set1_epi32(0b1111_1111_1111));
        let indices_1_4 = _mm256_extracti128_si256::<1>(input_indices);
        let scores_1_4 = _mm256_i32gather_epi64::<8>(lut, indices_1_4);
        let score_1_4 = _mm256_hadd_epi64(scores_1_4);

        let indices_5_7 = _mm256_extracti128_si256::<0>(input_indices);
        let scores_5_7 = _mm256_i32gather_epi64::<8>(lut, indices_5_7);
        let score = _mm256_add_epi64(scores_5_7, score_1_4);
        std::mem::transmute::<i64, u64>(_mm256_extract_epi64(score, 3))
    }
}

#[inline]
#[aoc(day21, part1, code_lut_simd)]
fn one_code_lut_simd(input: &str) -> u64 {
    code_lut_simd::<2>(input)
}

#[inline]
#[aoc(day21, part2, code_lut_simd)]
fn two_code_lut_simd(input: &str) -> u64 {
    code_lut_simd::<25>(input)
}

const LUT_25: [u64; 4096] = build_code_lut(25);
const LUT_2: [u64; 4096] = build_code_lut(2);

#[target_feature(enable = "avx2")]
#[inline]
unsafe fn code_lut_simd_asm<const DEPTH: usize>(input: &str) -> u64 {
    let input = input.as_ptr();
    let lut = if DEPTH == 2 { &LUT_2 } else { &LUT_25 }.as_ptr();

    unsafe {
        let input_offsets = _mm256_set_epi32(0, 5, 10, 15, 20, 20, 20, 20);

        let score: __m256i;
        std::arch::asm!(
            "vpcmpeqd {simd_tmp}, {simd_tmp}, {simd_tmp}", // set all to 1
            "vpgatherdd {simd1}, ymmword ptr [{input}+{simd2}*1], {simd_tmp}",
            "vpsrld {simd2}, {simd1}, 12",
            "vpxor {simd1}, {simd2}, {simd1}",
            "vpcmpeqd {simd_tmp}, {simd_tmp}, {simd_tmp}", // set all to 1 again, was cleared by gather
            "vpsrld {simd2}, {simd_tmp}, {index_mask_shift}",
            "vpand {simd1}, {simd2}, {simd1}",
            "vextracti128 {indices1}, {simd1}, 1",
            "vextracti128 {indices2}, {simd1}, 0",
            "vpcmpeqd {simd_tmp}, {simd_tmp}, {simd_tmp}",
            "vpcmpeqd {simd_tmp2}, {simd_tmp2}, {simd_tmp2}",
            "vpgatherdq {simd1}, ymmword ptr [{lut}+{indices1}*8], {simd_tmp}",
            "vpgatherdq {simd2}, ymmword ptr [{lut}+{indices2}*8], {simd_tmp2}",
            "vpermq {simd_tmp}, {simd1}, {shuf1}",
            "vpaddq {simd1}, {simd_tmp}, {simd1}",
            "vpermq {simd_tmp}, {simd1}, {shuf2}",
            "vpaddq {simd1}, {simd_tmp}, {simd1}",
            "vpaddd {simd1}, {simd2}, {simd1}",

            simd1 = out(ymm_reg) score,
            lut = in(reg) lut,
            input = in(reg) input,
            simd2 = in(ymm_reg) input_offsets,
            simd_tmp = out(ymm_reg) _,
            simd_tmp2 = out(ymm_reg) _,
            indices1 = out(xmm_reg) _,
            indices2 = out(xmm_reg) _,
            index_mask_shift = const 32 - 12,
            shuf1 = const 0b00_01_10_11,
            shuf2 = const 0b01_00_00_01,
        );

        std::mem::transmute::<i64, u64>(_mm256_extract_epi64(score, 3))
    }
}

#[inline]
#[aoc(day21, part1, code_lut_simd_asm)]
fn one_code_lut_simd_asm(input: &str) -> u64 {
    unsafe { code_lut_simd_asm::<2>(input) }
}

#[inline]
#[aoc(day21, part2, code_lut_simd_asm)]
fn two_code_lut_simd_asm(input: &str) -> u64 {
    unsafe { code_lut_simd_asm::<25>(input) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1() {
        let moves = numpad_moves(b"029A");
        dbg!(&moves);
        assert_eq!(moves.len(), "<A^A^^>AvvvA".len())
    }

    #[test]
    fn test_stage2() {
        let numpad_moves = numpad_moves(b"029A");
        dbg!(&numpad_moves);
        let dpad1_moves = dpad_moves(&numpad_moves);
        dbg!(&dpad1_moves);
        assert_eq!(dpad1_moves.len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len())
    }

    #[test]
    fn test_part1() {
        let input = parse(include_str!("test.txt"));
        assert_eq!(one(&input), 126384);
    }

    #[test]
    fn test_pub_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 126384);
    }

    #[test]
    fn test_pub_part2() {
        let input = include_str!("test.txt");
        assert_eq!(part2(input), 154115708116294);
    }

    #[test]
    fn test_numpad_one_move_array() {
        for from in NUMPAD_KEYS {
            for to in NUMPAD_KEYS {
                let (path, len) = numpad_one_move_array(from, to);
                let vec_path = numpad_one_move(from, to);
                assert_eq!(len, vec_path.len());
                assert_eq!(
                    &path[..len].iter().map(|x| x.unwrap()).collect::<Vec<_>>(),
                    vec_path.as_slice()
                );
            }
        }
    }
}
