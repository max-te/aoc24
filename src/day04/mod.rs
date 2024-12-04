use crate::Solutions;
use itertools::Itertools;
use lib_aoc::prelude::*;

impl Solution<DAY_04> for Solutions {
    type Input<'i> = &'i [u8];
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle.as_bytes()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        let mut h_machine = XmasCountMachine::new();
        let mut v_machines = Vec::new();
        let mut diag_r_machines = Vec::new();
        let mut diag_l_machines = Vec::new();

        let mut iter = input.iter();
        for ch in iter.by_ref() {
            h_machine.consume(*ch);

            let mut new_machine = XmasCountMachine::new();
            new_machine.consume(*ch);

            v_machines.push(new_machine.clone());
            diag_r_machines.push(new_machine.clone());
            diag_l_machines.push(new_machine);

            if *ch == b'\n' {
                break;
            }
        }

        let mut diag_r_index = diag_r_machines.len() - 1;
        let mut diag_l_index = 1;
        let mut v_index = 0;

        for ch in iter {
            h_machine.consume(*ch);

            // dbg!(&diag_l_index, &diag_r_index, &v_index);
            diag_l_machines[diag_l_index].consume(*ch);
            diag_r_machines[diag_r_index].consume(*ch);
            v_machines[v_index].consume(*ch);

            v_index = (v_index + 1) % v_machines.len();

            if *ch == b'\n' {
                diag_l_index = (diag_l_index + 2) % diag_l_machines.len();
                // diag_r_index = diag_r_index;
            } else {
                diag_l_index = (diag_l_index + 1) % diag_l_machines.len();
                diag_r_index = (diag_r_index + 1) % diag_r_machines.len();
            }
        }
        // dbg!(&h_machine, &v_machines, &diag_l_machines, &diag_r_machines);
        h_machine.count
            + v_machines.iter().map(|m| m.count).sum::<usize>()
            + diag_l_machines.iter().map(|m| m.count).sum::<usize>()
            + diag_r_machines.iter().map(|m| m.count).sum::<usize>()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        let mut count = 0;
        let lines = input.split(|x| *x == b'\n');
        for (before, middle, after) in lines.tuple_windows() {
            if after.is_empty() {
                continue;
            }
            for center in 1usize..(middle.len() - 1) {
                if (middle[center] == b'A')
                    && (before[center - 1] == b'S' && after[center + 1] == b'M'
                        || before[center - 1] == b'M' && after[center + 1] == b'S')
                    && (before[center + 1] == b'S' && after[center - 1] == b'M'
                        || before[center + 1] == b'M' && after[center - 1] == b'S')
                {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Debug, Clone, Copy)]
enum XmasReadState {
    Init,
    ReadX,
    ReadXM,
    ReadXMA,
    ReadS,
    ReadSA,
    ReadSAM,
}

#[derive(Debug, Clone)]
struct XmasCountMachine {
    read_state: XmasReadState,
    count: usize,
}

impl XmasCountMachine {
    fn new() -> Self {
        XmasCountMachine {
            read_state: XmasReadState::Init,
            count: 0,
        }
    }

    fn consume(&mut self, ch: u8) {
        self.read_state = match (self.read_state, ch) {
            (XmasReadState::ReadSAM, b'X') => {
                self.count += 1;
                XmasReadState::ReadX
            }
            (XmasReadState::ReadXMA, b'S') => {
                self.count += 1;
                XmasReadState::ReadS
            }
            (_, b'X') => XmasReadState::ReadX,
            (XmasReadState::ReadX, b'M') => XmasReadState::ReadXM,
            (XmasReadState::ReadXM, b'A') => XmasReadState::ReadXMA,
            (_, b'S') => XmasReadState::ReadS,
            (XmasReadState::ReadS, b'A') => XmasReadState::ReadSA,
            (XmasReadState::ReadSA, b'M') => XmasReadState::ReadSAM,
            _ => XmasReadState::Init,
        };
    }
}

impl Test<DAY_04> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 18,
            PART_TWO => 9,
        }
    }
}

derive_tests!(Solutions, DAY_04);

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn count_xmasamx<'a>(along: impl Iterator<Item = &'a u8>) -> usize {
        let mut counter = XmasCountMachine::new();
        for ch in along {
            counter.consume(*ch);
            dbg!(ch);
            dbg!(counter.read_state);
        }
        counter.count
    }

    #[test]
    fn counts_xmasses() {
        let input = b"XMASSXXMAS XMMASSS\nXMAS";
        let count = count_xmasamx(input.into_iter());
        assert_eq!(count, 3);
    }

    #[test]
    fn counts_samxes() {
        let input = b"XMASSXXMAS XMMASSS\nXMAS";
        let count = count_xmasamx(input.into_iter().rev());
        assert_eq!(count, 3);
    }

    #[test]
    fn counts_overlapping() {
        let input = b"XMASAMXMAS";
        let count = count_xmasamx(input.into_iter().rev());
        assert_eq!(count, 3);
    }
}
