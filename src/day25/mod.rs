use aoc_runner_derive::aoc;

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for schematic in input.split("\n\n").map(str::as_bytes) {
        if schematic[0] == b'#' {
            let mut lock = [0u8; 5];
            for r in 1..=5 {
                for c in 0..5 {
                    if schematic[r * 6 + c] == b'#' {
                        lock[c] += 1;
                    }
                }
            }
            locks.push(lock);
        } else {
            let mut key = [5u8; 5];
            for r in 1..=5 {
                for c in 0..5 {
                    if schematic[r * 6 + c] == b'.' {
                        key[c] -= 1;
                    }
                }
            }
            keys.push(key);
        }
    }

    let mut res = 0;
    for key in keys {
        for lock in &locks {
            if key_fits_lock(&key, &lock) {
                res += 1;
            }
        }
    }

    res
}

pub fn part2(_input: &str) -> usize {
    10
}

#[inline]
fn key_fits_lock(key: &[u8; 5], lock: &[u8; 5]) -> bool {
    key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("test.txt")), 3);
    }
}
