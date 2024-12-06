use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Output = usize;
type Coord = u16;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(Coord, Coord);

impl Point {
    fn in_range(&self, width: Coord, height: Coord) -> bool {
        (1..width).contains(&self.0) && (1..height).contains(&self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard(Point, Facing);

impl Guard {
    fn turn_right(&mut self) {
        self.1 = match self.1 {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        };
    }
}

type NaiveInput = (HashSet<Point>, Guard, Coord, Coord);

#[aoc_generator(day6, naive)]
fn parse_naive(puzzle: &str) -> NaiveInput {
    let puzzle = puzzle.as_bytes();
    let mut point = Point(1, 1);
    let mut obstacles = HashSet::new();
    let mut guard = None;
    let mut width = None;
    for ch in puzzle {
        match *ch {
            b'\n' => {
                width.get_or_insert(point.0);
                point.1 += 1;
                point.0 = 1;
                continue;
            }
            b'#' => {
                obstacles.insert(point.clone());
            }
            b'^' => {
                guard = Some(Guard(point.clone(), Facing::North));
            }
            b'>' => {
                guard = Some(Guard(point.clone(), Facing::East));
            }
            b'<' => {
                guard = Some(Guard(point.clone(), Facing::West));
            }
            b'v' => {
                guard = Some(Guard(point.clone(), Facing::South));
            }
            b'.' => {}
            _ => unreachable!(),
        }
        point.0 += 1;
    }

    (obstacles, guard.unwrap(), width.unwrap(), point.1)
}

#[aoc(day6, part1, naive)]
fn one_naive((obstacles, guard, width, height): &NaiveInput) -> Output {
    let mut guard = *guard;
    let mut visited = HashSet::new();
    while guard.0.in_range(*width, *height) {
        visited.insert(guard.0.clone());
        loop {
            let step_pos = guard.1.step(guard.0);
            if obstacles.contains(&step_pos) {
                guard.turn_right();
            } else {
                guard.0 = step_pos;
                break;
            }
        }
    }
    visited.len()
}

pub fn part1(puzzle: &str) -> Output {
    one_naive(&parse_naive(puzzle))
}

fn trace_base_path(
    obstacles: &HashSet<Point>,
    mut guard: Guard,
    width: Coord,
    height: Coord,
) -> HashSet<Guard> {
    let mut path = HashSet::new();
    while guard.0.in_range(width, height) {
        path.insert(guard);
        loop {
            let step_pos = guard.1.step(guard.0);
            if obstacles.contains(&step_pos) {
                guard.turn_right();
            } else {
                guard.0 = step_pos;
                break;
            }
        }
    }
    path
}

pub fn part2(puzzle: &str) -> Output {
    two_cast(&parse_naive(puzzle))
}

#[derive(Debug)]
struct EdgeMap {
    columns: Vec<Vec<Coord>>,
    rows: Vec<Vec<Coord>>,
}

impl EdgeMap {
    fn with_size(width: Coord, height: Coord) -> Self {
        Self {
            columns: (1..=width).map(|_| Vec::new()).collect(),
            rows: (1..=height).map(|_| Vec::new()).collect(),
        }
    }

    fn width(&self) -> Coord {
        self.columns.len() as Coord
    }

    fn height(&self) -> Coord {
        self.rows.len() as Coord
    }

    unsafe fn append(&mut self, point: Point) {
        self.columns[point.0 as usize - 1].push(point.1);
        self.rows[point.1 as usize - 1].push(point.0);
    }

    fn insert(&mut self, point: Point) {
        let index = self.columns[point.0 as usize - 1]
            .binary_search(&point.1)
            .unwrap_err();
        self.columns[point.0 as usize - 1].insert(index, point.1);
        let index = self.rows[point.1 as usize - 1]
            .binary_search(&point.0)
            .unwrap_err();
        self.rows[point.1 as usize - 1].insert(index, point.0);
    }

    fn remove(&mut self, point: &Point) {
        let index = self.columns[point.0 as usize - 1]
            .binary_search(&point.1)
            .unwrap();
        self.columns[point.0 as usize - 1].remove(index);
        let index = self.rows[point.1 as usize - 1].binary_search(&point.0).unwrap();
        self.rows[point.1 as usize - 1].remove(index);
    }

    fn contains(&self, point: &Point) -> bool {
        self.columns[point.0 as usize - 1]
            .binary_search(&point.1)
            .is_ok()
    }

    fn from_obstacles(obstacles: &HashSet<Point>, width: Coord, height: Coord) -> Self {
        let mut slf = Self::with_size(width, height);
        for point in obstacles {
            unsafe {
                // SAFETY: we sort the columns and rows afterwards
                slf.append(*point);
            }
        }
        slf.rows.iter_mut().for_each(|row| row.sort_unstable());
        slf.columns.iter_mut().for_each(|col| col.sort_unstable());
        slf
    }

    fn cast(&self, guard: Guard) -> Option<Point> {
        match guard.1 {
            Facing::North | Facing::South => {
                let col = &self.columns[guard.0 .0 as usize - 1];
                let idx = col.binary_search(&guard.0 .1).unwrap_err();
                if guard.1 == Facing::South {
                    if idx < col.len() {
                        Some(Point(guard.0 .0, col[idx] - 1))
                    } else {
                        None
                    }
                } else {
                    if idx > 0 {
                        Some(Point(guard.0 .0, col[idx - 1] + 1))
                    } else {
                        None
                    }
                }
            }
            Facing::East | Facing::West => {
                let row = &self.rows[guard.0 .1 as usize - 1];
                let idx = row.binary_search(&guard.0 .0).unwrap_err();
                if guard.1 == Facing::East {
                    if idx < row.len() {
                        Some(Point(row[idx] - 1, guard.0 .1))
                    } else {
                        None
                    }
                } else {
                    if idx > 0 {
                        Some(Point(row[idx - 1] + 1, guard.0 .1))
                    } else {
                        None
                    }
                }
            }
        }
    }
}

#[aoc(day6, part2, cast)]
fn two_cast((obstacles, guard, width, height): &NaiveInput) -> Output {
    let base_path = trace_base_path(&obstacles, *guard, *width, *height);
    let mut edge_map = EdgeMap::from_obstacles(&obstacles, *width, *height);
    let mut blockable = HashSet::new();
    for next in base_path.iter() {
        if next.0 == guard.0 {
            continue;
        }
        edge_map.insert(next.0);
        if cast_path(&edge_map, guard.clone()) {
            blockable.insert(next.0);
        }
        edge_map.remove(&next.0);
    }
    blockable.len()
}

fn _debug_draw_cast(edge_map: &EdgeMap, guard: Guard, path: &HashSet<Guard>) {
    for y in 1..=edge_map.height() {
        for x in 1..=edge_map.width() {
            let pos = Point(x, y);
            if edge_map.contains(&pos) {
                print!("#");
            } else if pos == guard.0 {
                print!("G");
            } else if path.iter().any(|g| g.0 == pos) {
                print!("+");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n\n");
}

fn cast_path(edge_map: &EdgeMap, mut guard: Guard) -> bool {
    let width = edge_map.width();
    let height = edge_map.height();
    let mut path = HashSet::new();
    while guard.0.in_range(width, height) {
        if !path.insert(guard) {
            return true;
        }
        match edge_map.cast(guard) {
            None => {
                return false;
            }
            Some(stop_pos) => {
                guard.0 = stop_pos;
                guard.turn_right();
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let res = part1(include_str!("test.txt"));
        assert_eq!(res, 41);
    }

    #[test]
    fn example2() {
        let res = part2(include_str!("test.txt"));
        assert_eq!(res, 6);
    }

}
