use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use smallvec::SmallVec;
use std::{
    collections::BTreeSet,
    fmt::Debug,
    hash::{Hash, Hasher},
};

type Output = u32;
type Coord = u16;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point(Coord, Coord);

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32((self.0 as u32) << 16 | self.1 as u32);
    }
}

struct Map<T> {
    data: Vec<T>,
    width: Coord,
    height: Coord,
}

impl<T> Map<T> {
    fn get(&self, point: &Point) -> &T {
        &self.data[(point.1 * self.width + point.0) as usize]
    }

    fn enumerate(&self) -> impl Iterator<Item = (Point, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, d)| (self.to_point(i), d))
    }

    #[inline(always)]
    fn to_point(&self, index: usize) -> Point {
        Point(index as Coord % self.width, index as Coord / self.width)
    }
}

impl<T> Debug for Map<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_fmt(format_args!("{:?}", self.get(&Point(x, y))))?;
            }
        }
        Ok(())
    }
}

impl<T> Clone for Map<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Map {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

impl<T: Default + Clone> Map<T> {
    fn remove(&mut self, point: &Point) -> T {
        let mut res = T::default();
        let cell = &mut self.data[point.1 as usize * self.width as usize + point.0 as usize];
        std::mem::swap(&mut res, cell);
        res
    }

    fn set(&mut self, point: &Point, mut value: T) -> T {
        let cell = &mut self.data[point.1 as usize * self.width as usize + point.0 as usize];
        std::mem::swap(&mut value, cell);
        value
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Object {
    Crate,
    Wall,
}

#[derive(Debug, Clone)]
struct Input {
    warehouse: Map<Option<Object>>,
    wilmot: Point,
    moves: Vec<Facing>,
}

#[aoc_generator(day15, part1)]
fn parse(puzzle: &str) -> Input {
    let puzzle = puzzle.as_bytes();
    let mut map = Vec::with_capacity(puzzle.len());
    let mut width = None;
    let mut height = None;
    let mut point = Point(0, 0);
    let mut wilmot = None;
    let mut moves = Vec::with_capacity(puzzle.len() / 2);
    for ch in puzzle {
        match *ch {
            b'\n' => {
                width.get_or_insert(point.0);
                point.1 += 1;
                point.0 = 0;
                continue;
            }
            b'#' => {
                map.push(Some(Object::Wall));
                height = Some(point.1 + 1);
            }
            b'O' => {
                map.push(Some(Object::Crate));
            }
            b'@' => {
                map.push(None);
                wilmot = Some(point.clone());
            }
            b'.' => {
                map.push(None);
            }
            b'^' => moves.push(Facing::North),
            b'v' => moves.push(Facing::South),
            b'<' => moves.push(Facing::West),
            b'>' => moves.push(Facing::East),
            _ => unreachable!(),
        }
        point.0 += 1;
    }
    Input {
        warehouse: Map {
            data: map,
            width: width.unwrap(),
            height: height.unwrap(),
        },
        wilmot: wilmot.unwrap(),
        moves,
    }
}

fn _debug_draw(warehouse: &Map<Option<Object>>, wilmot: Point) {
    let width = warehouse.width;
    let height = warehouse.height;
    for y in 0..height {
        for x in 0..width {
            if *warehouse.get(&Point(x, y)) == Some(Object::Wall) {
                print!("#");
            } else if *warehouse.get(&Point(x, y)) == Some(Object::Crate) {
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
        _debug_draw(&warehouse, wilmot);
        #[cfg(debug_assertions)]
        dbg!(&facing);
        let step_target = facing.step(wilmot);
        let mut push_end = step_target.clone();
        while *warehouse.get(&push_end) == Some(Object::Crate) {
            push_end = facing.step(push_end);
        }
        if *warehouse.get(&push_end) == Some(Object::Wall) {
            continue;
        } else {
            wilmot = step_target;
            warehouse.remove(&step_target).inspect(|o| {
                warehouse.set(&push_end, Some(*o));
            });
        }
    }

    let mut score = 0;
    for (point, object) in warehouse.enumerate() {
        if *object == Some(Object::Crate) {
            score += (100 * (point.1) + (point.0)) as u32;
        }
    }
    score
}

pub fn part1(puzzle: &str) -> Output {
    one(&parse(puzzle))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Object2 {
    CrateLeft,
    CrateRight,
    Wall,
}

#[derive(Debug, Clone)]
struct Input2 {
    warehouse: Map<Option<Object2>>,
    wilmot: Point,
    moves: Vec<Facing>,
}

#[aoc_generator(day15, part2)]
fn parse2(puzzle: &str) -> Input2 {
    let puzzle = puzzle.as_bytes();
    let mut map = Vec::with_capacity(puzzle.len());
    let mut width = None;
    let mut height = 0;
    let mut point = Point(0, 0);
    let mut wilmot = None;
    let mut moves = Vec::with_capacity(puzzle.len() / 2);
    for ch in puzzle {
        match *ch {
            b'\n' => {
                width.get_or_insert(point.0);
                point.1 += 1;
                point.0 = 0;
                continue;
            }
            b'#' => {
                map.extend_from_slice(&[Some(Object2::Wall), Some(Object2::Wall)]);
                height = point.1 + 1;
            }
            b'O' => {
                map.extend_from_slice(&[Some(Object2::CrateLeft), Some(Object2::CrateRight)]);
            }
            b'@' => {
                map.extend_from_slice(&[None, None]);
                wilmot = Some(point.clone());
            }
            b'.' => {
                map.extend_from_slice(&[None, None]);
            }
            b'^' => moves.push(Facing::North),
            b'v' => moves.push(Facing::South),
            b'<' => moves.push(Facing::West),
            b'>' => moves.push(Facing::East),
            _ => unreachable!(),
        }
        point.0 += 2;
    }
    Input2 {
        warehouse: Map {
            data: map,
            width: width.unwrap(),
            height: height,
        },
        wilmot: wilmot.unwrap(),
        moves,
    }
}

pub fn part2(puzzle: &str) -> Output {
    two(&parse2(puzzle))
}

#[aoc(day15, part2)]
fn two(input: &Input2) -> Output {
    let mut warehouse = input.warehouse.clone();
    let mut wilmot = input.wilmot.clone();
    #[cfg(debug_assertions)]
    _debug_draw_2(&warehouse, wilmot);

    for facing in input.moves.iter() {
        #[cfg(debug_assertions)]
        dbg!(&facing);

        let step_target = facing.step(wilmot);

        match *facing {
            Facing::North | Facing::South => {
                let could_move = push_crates(&mut warehouse, step_target, *facing);
                if could_move {
                    wilmot = step_target;
                }
            }
            Facing::West | Facing::East => {
                let mut push_end = step_target.clone();
                while matches!(
                    *warehouse.get(&push_end),
                    Some(Object2::CrateLeft) | Some(Object2::CrateRight)
                ) {
                    push_end = facing.step(push_end);
                }
                if *warehouse.get(&push_end) == Some(Object2::Wall) {
                    continue;
                } else if push_end == step_target {
                    wilmot = step_target;
                } else {
                    wilmot = step_target;
                    let mut pushed_point = step_target.clone();
                    let mut pushed_object = warehouse.remove(&pushed_point);
                    while pushed_object.is_some() {
                        pushed_point = facing.step(pushed_point);
                        pushed_object = warehouse.set(&pushed_point, pushed_object);
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        _debug_draw_2(&warehouse, wilmot);
    }

    let mut score = 0;
    for (point, object) in warehouse.enumerate() {
        if *object == Some(Object2::CrateLeft) {
            score += (100 * (point.1) + (point.0)) as u32;
        }
    }
    score
}

fn push_crates(warehouse: &mut Map<Option<Object2>>, step_target: Point, facing: Facing) -> bool {
    let mut push_front: SmallVec<[Point; 32]> = SmallVec::new();
    let mut to_push = BTreeSet::new();
    match warehouse.get(&step_target) {
        Some(Object2::Wall) => return false,
        None => return true,
        Some(Object2::CrateLeft) => {
            push_front.reserve(16);
            push_front.push(step_target);
            push_front.push(Facing::East.step(step_target));
        }
        Some(Object2::CrateRight) => {
            push_front.reserve(16);
            push_front.push(step_target);
            push_front.push(Facing::West.step(step_target));
        }
    }
    while !push_front.is_empty() {
        #[cfg(debug_assertions)]
        dbg!(&step_target, &push_front, &to_push);
        let point = push_front.pop().unwrap();
        let push_target = facing.step(point);
        match warehouse.get(&push_target) {
            Some(Object2::Wall) => return false,
            None => {}
            Some(Object2::CrateLeft) => {
                push_front.push(push_target);
                push_front.push(Facing::East.step(push_target));
            }
            Some(Object2::CrateRight) => {
                push_front.push(push_target);
                push_front.push(Facing::West.step(push_target));
            }
        }
        to_push.insert(point);
    }

    if facing == Facing::South {
        for point in to_push.into_iter().rev() {
            let push_target = facing.step(point);
            let from = warehouse.remove(&point);
            warehouse
                .set(&push_target, from)
                .inspect(|o| unreachable!("Missed a {o:?} while pushing"));
        }
    } else {
        debug_assert!(facing == Facing::North);
        for point in to_push.into_iter() {
            let push_target = facing.step(point);
            let from = warehouse.remove(&point);
            warehouse
                .set(&push_target, from)
                .inspect(|o| unreachable!("Missed a {o:?} while pushing"));
        }
    }
    true
}

fn _debug_draw_2(warehouse: &Map<Option<Object2>>, wilmot: Point) {
    let width = warehouse.width;
    let height = warehouse.height;
    for y in 0..height {
        for x in 0..width {
            if warehouse.get(&Point(x, y)) == &Some(Object2::Wall) {
                print!("#");
            } else if warehouse.get(&Point(x, y)) == &Some(Object2::CrateLeft) {
                print!("[");
            } else if warehouse.get(&Point(x, y)) == &Some(Object2::CrateRight) {
                print!("]");
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

    #[test]
    fn example2() {
        let res = part2(include_str!("test.txt"));
        assert_eq!(res, 9021);
    }
}
