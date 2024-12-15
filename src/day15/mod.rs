use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::hash::{Hash, Hasher};

type Output = u32;
type Coord = u16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(Coord, Coord);

impl Point {
    fn in_range(&self, width: Coord, height: Coord) -> bool {
        (0..width).contains(&self.0) && (0..height).contains(&self.1)
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32((self.0 as u32) << 16 | self.1 as u32);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    North,
    South,
    West,
    East,
}

impl Facing {
    fn step(self, point: Point) -> Point {
        match self {
            Facing::North => Point(point.0, point.1 - 1),
            Facing::South => Point(point.0, point.1 + 1),
            Facing::West => Point(point.0 - 1, point.1),
            Facing::East => Point(point.0 + 1, point.1),
        }
    }

    fn opposite(self) -> Facing {
        match self {
            Facing::North => Facing::South,
            Facing::South => Facing::North,
            Facing::West => Facing::East,
            Facing::East => Facing::West,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Object {
    Crate,
    Wall,
}

#[derive(Debug, Clone)]
struct Input {
    warehouse: FxHashMap<Point, Object>,
    wilmot: Point,
    moves: Vec<Facing>,
}

#[aoc_generator(day15)]
fn parse(puzzle: &str) -> Input {
    let puzzle = puzzle.as_bytes();
    let mut warehouse =
        FxHashMap::with_capacity_and_hasher(puzzle.len() / 2, FxBuildHasher::default());
    let mut point = Point(0, 0);
    let mut wilmot = None;
    let mut moves = Vec::with_capacity(puzzle.len() / 2);
    for ch in puzzle {
        match *ch {
            b'\n' => {
                point.1 += 1;
                point.0 = 0;
                continue;
            }
            b'#' => {
                warehouse.insert(point.clone(), Object::Wall);
            }
            b'O' => {
                warehouse.insert(point.clone(), Object::Crate);
            }
            b'@' => {
                wilmot = Some(point.clone());
            }
            b'.' => {}
            b'^' => moves.push(Facing::North),
            b'v' => moves.push(Facing::South),
            b'<' => moves.push(Facing::West),
            b'>' => moves.push(Facing::East),
            _ => unreachable!(),
        }
        point.0 += 1;
    }
    Input {
        warehouse,
        wilmot: wilmot.unwrap(),
        moves,
    }
}

fn debug_draw(warehouse: &FxHashMap<Point, Object>, wilmot: Point) {
    let width = warehouse.keys().map(|p| p.0).max().unwrap() + 1;
    let height = warehouse.keys().map(|p| p.1).max().unwrap() + 1;
    for y in 0..height {
        for x in 0..width {
            if warehouse.get(&Point(x, y)) == Some(&Object::Wall) {
                print!("#");
            } else if warehouse.get(&Point(x, y)) == Some(&Object::Crate) {
                print!("O");
            } else if Point(x, y) == wilmot {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[aoc(day15, part1)]
fn one(input: &Input) -> Output {
    let mut warehouse = input.warehouse.clone();
    let mut wilmot = input.wilmot.clone();
    for facing in input.moves.iter() {
        #[cfg(debug_assertions)]
        dbg!(&facing);
        let step_target = facing.step(wilmot);
        let mut push_end = step_target.clone();
        while warehouse.get(&push_end) == Some(&Object::Crate) {
            push_end = facing.step(push_end);
        }
        if warehouse.get(&push_end) == Some(&Object::Wall) {
            continue;
        } else {
            wilmot = step_target;
            warehouse.remove(&step_target).inspect(|o| {
                warehouse.insert(push_end, *o);
            });
        }
        #[cfg(debug_assertions)]
        debug_draw(&warehouse, wilmot);
    }

    let mut score = 0;
    for (point, object) in warehouse.iter() {
        if *object == Object::Crate {
            score += (100 * (point.1) + (point.0)) as u32;
        }
    }
    score
}

pub fn part1(puzzle: &str) -> Output {
    one(&parse(puzzle))
}

pub fn part2(puzzle: &str) -> Output {
    two(&parse(puzzle))
}

#[aoc(day15, part2)]
fn two(input: &Input) -> Output {
    todo!()
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, 10092);
    }

    #[test]
    fn example1_small() {
        let res = part1(include_str!("small.txt"));
        assert_eq!(res, 2028);
    }

    // #[test]
    // fn example2() {
    //     let res = part2(include_str!("test.txt"));
    //     assert_eq!(res, todo!());
    // }
}
