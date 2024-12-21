use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;

type Input = [[u8; 4]; 5];

#[aoc_generator(day21)]
pub fn parse(input: &str) -> Input {
    let input = input.as_bytes();
    let mut result = [[0; 4]; 5];
    for i in 0..5 {
        result[i].copy_from_slice(&input[i * 5..i * 5 + 4]);
    }
    result
}

#[aoc(day21, part1, naive)]
pub fn part1(input: &Input) -> usize {
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
pub fn part1_recursive(input: &Input) -> usize {
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
pub fn part2(input: &Input) -> usize {
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

fn numpad_position(key: u8) -> (usize, usize) {
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

fn numpad_moves(code: &[u8; 4]) -> Vec<DPadPress> {
    let mut moves = Vec::new();
    let mut pos = numpad_position(b'A');
    for target_key in code {
        let target_pos = numpad_position(*target_key);

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
                moves.extend_from_slice(v_moves);
            } else {
                // First vertical, then right
                moves.extend_from_slice(v_moves);
                for _ in 0..target_pos.0 - pos.0 {
                    moves.push(DPadPress::Right);
                }
            }
        } else if pos.1 == 3 && target_pos.0 == 0 {
            // First up, so we don't crash
            moves.extend_from_slice(v_moves);
            for _ in 0..pos.0 - target_pos.0 {
                moves.push(DPadPress::Left);
            }
        } else {
            // First left, then vertical
            for _ in 0..pos.0 - target_pos.0 {
                moves.push(DPadPress::Left);
            }
            moves.extend_from_slice(v_moves);
        }
        moves.push(DPadPress::Activate);
        pos = target_pos;
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
    if depth == 0 {
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
        len += dpad_one_move_recursive(last_pos, next, dpad_count - 1, dpad_memo);
        last_pos = next;
    }
    len
}

#[aoc(day21, part2, lut)]
pub fn part2_lut(input: &Input) -> usize {
    let mut res = 0;
    let lut = build_dpad_lut(25);
    for code in input {
        let move_count = input_code_lut(*code, &lut);
        let value = (code[0] - b'0') as usize * 100
            + (code[1] - b'0') as usize * 10
            + (code[2] - b'0') as usize;
        res += value * move_count;
    }
    res
}

fn build_dpad_lut(depth: usize) -> FxHashMap<(DPadPress, DPadPress), usize> {
    let mut lut = FxHashMap::default();
    if depth == 0 {
        for &from in DPadPress::values() {
            for &to in DPadPress::values() {
                let path = dpad_one_move(from, to);
                lut.insert((from, to), path.len());
            }
        }
    } else {
        let prev_lut = build_dpad_lut(depth - 1);
        for &from in DPadPress::values() {
            for &to in DPadPress::values() {
                let path = dpad_one_move(from, to);
                if depth == 0 {
                    lut.insert((from, to), path.len());
                } else {
                    let mut len = 0;
                    let mut last_pos = DPadPress::Activate;
                    for next in path {
                        len += prev_lut[&(last_pos, *next)];
                        last_pos = *next;
                    }
                    lut.insert((from, to), len);
                }
            }
        }
    }
    lut
}

fn input_code_lut(code: [u8; 4], dpad_lut: &FxHashMap<(DPadPress, DPadPress), usize>) -> usize {
    let numpad = numpad_moves(&code);
    let mut len = 0;
    let mut last_pos = DPadPress::Activate;
    for next in numpad {
        len += dpad_lut[&(last_pos, next)];
        last_pos = next;
    }
    len
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
        assert_eq!(part1(&input), 126384);
    }
}
