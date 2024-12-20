use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::{matrix::Matrix, prelude::bfs};

use crate::util::first_line_length;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tile {
    #[default]
    Wall,
    Track(usize),
}

type Point = (usize, usize);
type Input = (Matrix<Tile>, Point, Point);

#[aoc_generator(day20)]
fn parse(input: &str) -> Input {
    let input = input.as_bytes().trim_ascii_end();
    let mut start = None;
    let mut finish = None;
    let mut map = Matrix::<Tile>::new_square(first_line_length(input), Tile::Wall);
    for (x, row) in input.split(|ch| *ch == b'\n').enumerate() {
        for (y, &ch) in row.iter().enumerate() {
            match ch {
                b'.' => map[(x, y)] = Tile::Track(usize::MAX),
                b'S' => {
                    start = Some((x, y));
                    map[(x, y)] = Tile::Track(usize::MAX)
                }
                b'E' => {
                    finish = Some((x, y));
                    map[(x, y)] = Tile::Track(usize::MAX)
                }
                b'#' => (),
                _ => unreachable!(),
            }
        }
    }

    (map, start.unwrap(), finish.unwrap())
}

const SHORTCUT_DIRECTIONS: [(isize, isize); 8] = [
    (-2, 0),
    (-1, -1),
    (0, -2),
    (1, -1),
    (2, 0),
    (1, 1),
    (0, 2),
    (-1, 1),
];

fn one_inner((map, start, finish): &Input, min_save: usize) -> usize {
    let mut map = map.clone();
    let track = bfs(
        start,
        |t| {
            map.neighbours(*t, false)
                .filter(|t| matches!(map[*t], Tile::Track(_)))
        },
        |t| t == finish,
    )
    .unwrap();
    let mut shortcuts = 0;

    for (time, pos) in track.iter().enumerate() {
        map[*pos] = Tile::Track(time);
        for target in SHORTCUT_DIRECTIONS
            .iter()
            .filter_map(|dir| map.move_in_direction(*pos, *dir))
        {
            if let Tile::Track(target_time) = map[target] {
                if time.saturating_sub(target_time) >= 2 + min_save {
                    #[cfg(test)]
                    eprintln!(
                        "Shortcut to {pos:?} from {target:?} to time {time} from {target_time}"
                    );
                    shortcuts += 1;
                };
            }
        }
    }
    shortcuts
}

#[aoc(day20, part1)]
fn one(input: &Input) -> usize {
    one_inner(input, 100)
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = one_inner(&parse(include_str!("test.txt")), 20);
        assert_eq!(res, 5);
    }
}
