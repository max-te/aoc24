use aoc_runner_derive::{aoc, aoc_generator};

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

#[aoc(day21, part1)]
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

#[aoc(day21, part2)]
pub fn part2(input: &Input) -> usize {
    let mut res = 0;
    for code in input {
        let move_count = input_code_25(*code).len();
        let value = (code[0] - b'0') as usize * 100
            + (code[1] - b'0') as usize * 10
            + (code[2] - b'0') as usize;
        res += value * move_count;
    }
    res
}

fn input_code_25(code: [u8; 4]) -> Vec<DPadPress> {
    eprintln!("{}", String::from_utf8_lossy(&code));
    let stage1 = numpad_moves(&code);
    for mov in stage1.iter() {
        print!("{mov}");
    }
    let mut stage = stage1;
    for _ in 0..25 {
        stage = dpad_moves(&stage);
        todo!("This will blow up exponetially! Solve differently");
        // for mov in stage.iter() {
        //     print!("{mov}");
        // }
    }
    stage
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DPadPress {
    Up,
    Down,
    Left,
    Right,
    Activate,
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
        } else if target_pos.0 < pos.0 {
            if pos.1 == 3 && target_pos.0 == 0 {
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
        } else {
            moves.extend_from_slice(v_moves);
        }
        moves.push(DPadPress::Activate);
        pos = target_pos;
    }
    moves
}

fn dpad_position(key: DPadPress) -> (usize, usize) {
    match key {
        DPadPress::Up => (1, 0),
        DPadPress::Activate => (2, 0),

        DPadPress::Left => (0, 1),
        DPadPress::Down => (1, 1),
        DPadPress::Right => (2, 1),
    }
}

fn dpad_moves(code: &[DPadPress]) -> Vec<DPadPress> {
    let mut moves = Vec::new();
    let mut pos = dpad_position(DPadPress::Activate);
    for target_key in code {
        let target_pos = dpad_position(*target_key);
        // eprintln!("{:?} -> {:?} {:?}", pos, target_pos, target_key);

        if target_pos.1 > pos.1 && target_key == &DPadPress::Left {
            moves.push(DPadPress::Down);
        }

        if target_pos.0 > pos.0 {
            for _ in 0..target_pos.0 - pos.0 {
                moves.push(DPadPress::Right);
            }
        } else {
            for _ in 0..pos.0 - target_pos.0 {
                moves.push(DPadPress::Left);
            }
        }

        if target_pos.1 > pos.1 && target_key != &DPadPress::Left {
            moves.push(DPadPress::Down);
        }

        if target_pos.1 < pos.1 {
            moves.push(DPadPress::Up);
        }

        moves.push(DPadPress::Activate);
        pos = target_pos;
    }
    moves
}

fn input_code(code: [u8; 4]) -> Vec<DPadPress> {
    eprintln!("{}", String::from_utf8_lossy(&code));
    let stage1 = numpad_moves(&code);
    for mov in stage1.iter() {
        print!("{mov}");
    }
    println!("\n {} moves", stage1.len());
    let stage2 = dpad_moves(&stage1);
    for mov in stage2.iter() {
        print!("{mov}");
    }
    println!("\n {} moves", stage2.len());
    let stage3 = dpad_moves(&stage2);
    for mov in stage3.iter() {
        print!("{mov}");
    }
    println!("\n {} moves", stage3.len());
    stage3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1() {
        let moves = numpad_moves(&[b'0', b'2', b'9', b'A']);
        dbg!(&moves);
        assert_eq!(moves.len(), "<A^A^^>AvvvA".len())
    }

    #[test]
    fn test_stage2() {
        let numpad_moves = numpad_moves(&[b'0', b'2', b'9', b'A']);
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
