use std::{char, collections::HashMap, num::NonZeroUsize, ops::IndexMut, usize};

use aoc_runner_derive::{aoc, aoc_generator};

type Output = usize;
type Plant = u8;

struct Grid2D {
    data: Vec<Plant>,
    pad: usize,
    width: usize,
    height: usize,
}

impl Grid2D {
    fn new_from_newlines(data: Vec<Plant>) -> Self {
        let width = data.iter().position(|c| *c == b'\n').unwrap_or(data.len());
        let pad = 1;
        let height = (data.len() + pad) / (width + pad);
        Self {
            data,
            pad,
            width,
            height,
        }
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> Plant {
        self.data[y * (self.width + self.pad) + x]
    }

    #[inline]
    fn east_of(&self, x: usize, y: usize) -> Option<Plant> {
        if x <= 0 {
            None
        } else {
            Some(self.get(x - 1, y))
        }
    }

    #[inline]
    fn north_of(&self, x: usize, y: usize) -> Option<Plant> {
        if y <= 0 {
            None
        } else {
            Some(self.get(x, y - 1))
        }
    }
}

type Input = Grid2D;

#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    let input = input.as_bytes();
    Grid2D::new_from_newlines(input.to_vec())
}

type RegionId = usize;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Region {
    Active {
        id: RegionId,
        area: usize,
        perimeter: usize,
        plant: Plant,
    },
    Merged {
        id: RegionId,
        into: usize,
        plant: Plant,
    },
}

fn get_two_mut(
    regions: &mut Vec<Region>,
    id1: RegionId,
    id2: RegionId,
) -> (&mut Region, &mut Region) {
    debug_assert!(id1 != id2);
    if id1 < id2 {
        let (r1, r2) = regions.split_at_mut(id2);
        (&mut r1[id1], &mut r2[0])
    } else {
        let (r2, r1) = regions.split_at_mut(id1);
        (&mut r1[0], &mut r2[id2])
    }
}

#[aoc(day12, part1)]
fn one(plots: &Input) -> Output {
    let mut regions: Vec<Region> = Vec::new();

    let mut previous_row: Vec<RegionId> = Vec::with_capacity(plots.width);
    for y in 0..plots.height {
        let mut current_row = Vec::with_capacity(plots.width);
        for x in 0..plots.width {
            let this_plant = plots.get(x, y);
            let north_plant = plots.north_of(x, y);
            let east_plant = plots.east_of(x, y);
            if north_plant == Some(this_plant) {
                let mut north_region_id = previous_row[x];
                while let Region::Merged { into, .. } = regions[north_region_id] {
                    north_region_id = into;
                }

                if east_plant == Some(this_plant) {
                    let east_region_id = current_row[x - 1];
                    current_row.push(east_region_id);
                    if east_region_id == north_region_id {
                        let east_region = &mut regions[east_region_id];
                        match east_region {
                            Region::Active { area, .. } => {
                                *area += 1;
                                // perimeter unchanged
                            }
                            Region::Merged { .. } => unreachable!(),
                        }
                    } else {
                        let (east_region, north_region) =
                            get_two_mut(&mut regions, east_region_id, north_region_id);
                        match (east_region, &north_region) {
                            (
                                Region::Active {
                                    area: east_area,
                                    perimeter: east_perimeter,
                                    ..
                                },
                                Region::Active {
                                    area: north_area,
                                    perimeter: north_perimeter,
                                    ..
                                },
                            ) => {
                                *east_area += *north_area + 1;
                                *east_perimeter += *north_perimeter;
                            }
                            _ => unreachable!(),
                        }
                        *north_region = Region::Merged {
                            id: north_region_id,
                            into: east_region_id,
                            plant: this_plant,
                        };
                    }
                } else {
                    current_row.push(north_region_id);
                    let north_region = &mut regions[north_region_id];
                    match north_region {
                        Region::Active {
                            area, perimeter, ..
                        } => {
                            *area += 1;
                            *perimeter += 1; // West
                        }
                        Region::Merged { .. } => unreachable!(),
                    }
                }
            } else if x > 0 && east_plant == Some(this_plant) {
                let east_region_id = current_row[x - 1];
                current_row.push(east_region_id);
                let east_region = &mut regions[east_region_id];
                match east_region {
                    Region::Active {
                        area, perimeter, ..
                    } => {
                        *area += 1;
                        *perimeter += 1; // North
                    }
                    Region::Merged { .. } => unreachable!(),
                }
            } else {
                let new_region_id = regions.len();
                regions.push(Region::Active {
                    id: new_region_id,
                    area: 1,
                    perimeter: 2,
                    plant: this_plant,
                });
                current_row.push(new_region_id);
            }
            debug_assert!(current_row.len() == x + 1);
        }
        #[cfg(debug_assertions)]
        eprintln!("{current_row:3?}");
        previous_row = current_row;
    }

    #[cfg(debug_assertions)]
    {
        dbg!(&regions, plots.height, plots.width);
    }

    regions
        .iter()
        .map(|region| match region {
            Region::Active {
                id,
                plant,
                area,
                perimeter,
                ..
            } => {
                #[cfg(debug_assertions)]
                eprintln!(
                    "Region {id} of {} plants with price {} * {} = {}",
                    char::from_u32(*plant as u32).unwrap(),
                    area,
                    perimeter * 2,
                    area * 2 * perimeter
                );
                *area * 2 * *perimeter
            }
            Region::Merged { .. } => 0,
        })
        .sum()
}

#[aoc(day12, part2)]
fn two(plots: &Input) -> Output {
    todo!()
}

pub fn part1(puzzle: &str) -> Output {
    one(&parse(puzzle))
}

pub fn part2(puzzle: &str) -> Output {
    two(&parse(puzzle))
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1_large() {
        let res = one(&parse(include_str!("test.txt")));
        assert_eq!(res, 1930);
    }

    #[test]
    fn example1_medium() {
        let res = one(&parse(include_str!("test_medium.txt")));
        assert_eq!(res, 772);
    }

    #[test]
    fn example1_small() {
        let res = one(&parse(include_str!("test_small.txt")));
        assert_eq!(res, 140);
    }

    // #[test]
    // fn example2() {
    //     let res = two(&parse(include_str!("test.txt")));
    //     assert_eq!(res, 81);
    // }
}
