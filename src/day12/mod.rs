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
    fn west_of(&self, x: usize, y: usize) -> Option<Plant> {
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

    #[inline]
    fn northwest_of(&self, x: usize, y: usize) -> Option<Plant> {
        if x <= 0 || y <= 0 {
            None
        } else {
            Some(self.get(x - 1, y - 1))
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
        #[cfg(debug_assertions)]
        _id: RegionId,
        area: usize,
        /// For part 2 this represents the number of corners / sides
        perimeter: usize,
        #[cfg(debug_assertions)]
        _plant: Plant,
    },
    Merged {
        #[cfg(debug_assertions)]
        _id: RegionId,
        into: usize,
        #[cfg(debug_assertions)]
        _plant: Plant,
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
            let west_plant = plots.west_of(x, y);
            if north_plant == Some(this_plant) {
                let mut north_region_id = previous_row[x];
                while let Region::Merged { into, .. } = regions[north_region_id] {
                    north_region_id = into;
                }

                if west_plant == Some(this_plant) {
                    let west_region_id = current_row[x - 1];
                    current_row.push(west_region_id);
                    if west_region_id == north_region_id {
                        let west_region = &mut regions[west_region_id];
                        match west_region {
                            Region::Active { area, .. } => {
                                *area += 1;
                                // perimeter unchanged
                            }
                            Region::Merged { .. } => unreachable!(),
                        }
                    } else {
                        let (west_region, north_region) =
                            get_two_mut(&mut regions, west_region_id, north_region_id);
                        match (west_region, &north_region) {
                            (
                                Region::Active {
                                    area: west_area,
                                    perimeter: west_perimeter,
                                    ..
                                },
                                Region::Active {
                                    area: north_area,
                                    perimeter: north_perimeter,
                                    ..
                                },
                            ) => {
                                *west_area += *north_area + 1;
                                *west_perimeter += *north_perimeter;
                            }
                            _ => unreachable!(),
                        }
                        *north_region = Region::Merged {
                            #[cfg(debug_assertions)]
                            _id: north_region_id,
                            into: west_region_id,
                            #[cfg(debug_assertions)]
                            _plant: this_plant,
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
            } else if x > 0 && west_plant == Some(this_plant) {
                let west_region_id = current_row[x - 1];
                current_row.push(west_region_id);
                let west_region = &mut regions[west_region_id];
                match west_region {
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
                    #[cfg(debug_assertions)]
                    _id: new_region_id,
                    area: 1,
                    perimeter: 2,
                    #[cfg(debug_assertions)]
                    _plant: this_plant,
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
                #[cfg(debug_assertions)]
                _id,
                #[cfg(debug_assertions)]
                _plant,
                area,
                perimeter,
                ..
            } => {
                #[cfg(debug_assertions)]
                eprintln!(
                    "Region {_id} of {} plants with price {} * {} = {}",
                    char::from_u32(*_plant as u32).unwrap(),
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

fn increment_region(
    regions: &mut Vec<Region>,
    region_id: RegionId,
    area_change: usize,
    perimeter_change: usize,
) {
    // let region_id = resolve_id(regions, region_id);
    let region = &mut regions[region_id];
    // eprintln!("{region_id}: a+{}, p+{}", area_change, perimeter_change);
    match region {
        Region::Active {
            area, perimeter, ..
        } => {
            *area += area_change;
            *perimeter += perimeter_change;
        }
        Region::Merged { .. } => panic!(
            "Can't increment merged region {region_id} {:?}",
            &regions[region_id]
        ),
    }
}

fn resolve_id(regions: &mut Vec<Region>, id1: RegionId) -> RegionId {
    let mut id = id1;
    while let Region::Merged { into, .. } = regions[id] {
        id = into;
    }
    id
}

#[aoc(day12, part2)]
fn two(plots: &Input) -> Output {
    let mut regions: Vec<Region> = Vec::new();

    let mut previous_row: Vec<RegionId> = Vec::with_capacity(plots.width);
    for y in 0..plots.height {
        let mut current_row = Vec::with_capacity(plots.width);
        for x in 0..plots.width {
            let this_plant = plots.get(x, y);
            if x > 0 && y > 0 {
                let north_plant = plots.north_of(x, y).unwrap();
                let west_plant = plots.west_of(x, y).unwrap();
                let northwest_plant = plots.northwest_of(x, y).unwrap();
                let western_region_id = current_row[x - 1];
                let northern_region_id = resolve_id(&mut regions, previous_row[x]);
                let northwestern_region_id = resolve_id(&mut regions, previous_row[x - 1]);

                // eprintln!("({x}, {y}) NW={northwestern_region_id:2} N={northern_region_id:2} W={western_region_id:2}");
                // eprintln!("{}{}\n{}{}", char::from_u32(northwest_plant as u32).unwrap(), char::from_u32(north_plant as u32).unwrap(), char::from_u32(west_plant as u32).unwrap(), char::from_u32(this_plant as u32).unwrap());

                match (
                    north_plant == this_plant,
                    northwest_plant == this_plant,
                    west_plant == this_plant,
                ) {
                    (false, _, false) => {
                        // eprintln!("Case 1");
                        // yx
                        // zT
                        // This is a northwest corner, open a new region
                        let new_region_id = regions.len();
                        regions.push(Region::Active {
                            #[cfg(debug_assertions)]
                            _id: new_region_id,
                            area: 1,
                            perimeter: 1,
                            #[cfg(debug_assertions)]
                            _plant: this_plant,
                        });
                        current_row.push(new_region_id);

                        if northwestern_region_id == northern_region_id
                            && western_region_id == northwestern_region_id
                        {
                            increment_region(&mut regions, northwestern_region_id, 0, 1);
                        } else if northwestern_region_id == northern_region_id {
                            increment_region(&mut regions, western_region_id, 0, 1);
                        } else if northwestern_region_id == western_region_id {
                            increment_region(&mut regions, northern_region_id, 0, 1);
                        } else {
                            increment_region(&mut regions, northern_region_id, 0, 1);
                            increment_region(&mut regions, northwestern_region_id, 0, 1);
                            increment_region(&mut regions, western_region_id, 0, 1);
                        }
                    }
                    (true, true, true) => {
                        // eprintln!("Case 2");
                        // TT
                        // TT
                        // This is an inner point, continue region
                        current_row.push(western_region_id);
                        increment_region(&mut regions, western_region_id, 1, 0);
                    }
                    (true, false, false) => {
                        // eprintln!("Case 3");
                        // yT
                        // xT
                        // This is a west edge, continue northern region
                        current_row.push(northern_region_id);
                        increment_region(&mut regions, northern_region_id, 1, 0);

                        if western_region_id != northwestern_region_id {
                            increment_region(&mut regions, western_region_id, 0, 1);
                            increment_region(&mut regions, northwestern_region_id, 0, 1);
                        }
                    }
                    (false, false, true) => {
                        // eprintln!("Case 4");
                        // yx
                        // TT
                        // This is a north edge, continue western region
                        current_row.push(western_region_id);
                        increment_region(&mut regions, western_region_id, 1, 0);

                        if northern_region_id != northwestern_region_id {
                            increment_region(&mut regions, northern_region_id, 0, 1);
                            increment_region(&mut regions, northwestern_region_id, 0, 1);
                        }
                    }
                    (true, true, false) => {
                        // eprintln!("Case 5");
                        // TT
                        // xT
                        // This is an inner northwest corner, continue northern region
                        current_row.push(northern_region_id);
                        increment_region(&mut regions, northern_region_id, 1, 1);
                        increment_region(&mut regions, western_region_id, 0, 1);
                    }
                    (false, true, true) => {
                        // eprintln!("Case 6");
                        // Tx
                        // TT
                        // This is an inner southwest corner, continue western region
                        current_row.push(western_region_id);
                        increment_region(&mut regions, western_region_id, 1, 1);
                        increment_region(&mut regions, northern_region_id, 0, 1);
                    }
                    (true, false, true) => {
                        // eprintln!("Case 7");
                        // yT
                        // TT
                        // This is an inner southwest corner, merge northern region into western region and continue
                        current_row.push(western_region_id);
                        increment_region(&mut regions, western_region_id, 1, 1);

                        if northern_region_id != western_region_id {
                            let (northern_area, northern_perimeter) =
                                match &regions[northern_region_id] {
                                    Region::Active {
                                        area, perimeter, ..
                                    } => (*area, *perimeter),
                                    Region::Merged { .. } => unreachable!(),
                                };
                            increment_region(
                                &mut regions,
                                western_region_id,
                                northern_area,
                                northern_perimeter,
                            );
                            regions[northern_region_id] = Region::Merged {
                                #[cfg(debug_assertions)]
                                _id: northern_region_id,
                                into: western_region_id,
                                #[cfg(debug_assertions)]
                                _plant: this_plant,
                            };
                        }

                        increment_region(&mut regions, northwestern_region_id, 0, 1);
                    }
                }
            } else if y == 0 && x == 0 {
                // Northwest corner
                let new_region_id = regions.len();
                regions.push(Region::Active {
                    #[cfg(debug_assertions)]
                    _id: new_region_id,
                    area: 1,
                    perimeter: 1,
                    #[cfg(debug_assertions)]
                    _plant: this_plant,
                });
                current_row.push(new_region_id);
            } else if y == 0 {
                // North edge
                let west_plant = plots.west_of(x, y).unwrap();
                let western_region_id = current_row[x - 1];
                if west_plant == this_plant {
                    current_row.push(current_row[x - 1]);
                    increment_region(&mut regions, western_region_id, 1, 0);
                } else {
                    let new_region_id = regions.len();
                    regions.push(Region::Active {
                        #[cfg(debug_assertions)]
                        _id: new_region_id,
                        area: 1,
                        perimeter: 1,
                        #[cfg(debug_assertions)]
                        _plant: this_plant,
                    });
                    current_row.push(new_region_id);

                    increment_region(&mut regions, western_region_id, 0, 1);
                }
            } else if x == 0 {
                // West edge
                let north_plant = plots.north_of(x, y).unwrap();
                let northern_region_id = resolve_id(&mut regions, previous_row[x]);
                if north_plant == this_plant {
                    current_row.push(northern_region_id);
                    increment_region(&mut regions, northern_region_id, 1, 0);
                } else {
                    let new_region_id = regions.len();
                    regions.push(Region::Active {
                        #[cfg(debug_assertions)]
                        _id: new_region_id,
                        area: 1,
                        perimeter: 1,
                        #[cfg(debug_assertions)]
                        _plant: this_plant,
                    });
                    current_row.push(new_region_id);

                    increment_region(&mut regions, northern_region_id, 0, 1);
                }
            }

            // East corners on eastern edge
            if x == plots.width - 1 {
                // eprintln!("East edge ({x}, {y})");
                let region_id = current_row[x];
                if y == 0 {
                    increment_region(&mut regions, region_id, 0, 1);
                } else {
                    let north_region_id = resolve_id(&mut regions, previous_row[x]);
                    if region_id != north_region_id {
                        increment_region(&mut regions, region_id, 0, 1);
                        increment_region(&mut regions, north_region_id, 0, 1);
                    }
                }
            }
            // South corners on southern edge
            if y == plots.height - 1 {
                // eprintln!("South edge ({x}, {y})");
                let region_id = current_row[x];
                if x == 0 || x == plots.width - 1 {
                    increment_region(&mut regions, region_id, 0, 1);
                }
                if x > 0 && region_id != current_row[x - 1] {
                    increment_region(&mut regions, region_id, 0, 1);
                    increment_region(&mut regions, current_row[x - 1], 0, 1);
                }
            }
        }
        previous_row = current_row;
    }

    // #[cfg(debug_assertions)]
    // {
    //     dbg!(&regions, plots.height, plots.width);
    // }

    regions
        .iter()
        .map(|region| match region {
            Region::Active {
                #[cfg(debug_assertions)]
                _id,
                #[cfg(debug_assertions)]
                _plant,
                area,
                perimeter,
                ..
            } => {
                #[cfg(debug_assertions)]
                eprintln!(
                    "Region {_id} of {} plants with price {} * {} = {}",
                    char::from_u32(*_plant as u32).unwrap(),
                    area,
                    perimeter,
                    area * perimeter
                );
                *area * *perimeter
            }
            Region::Merged { .. } => 0,
        })
        .sum()
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

    #[test]
    fn example2_small() {
        let res = two(&parse(include_str!("test_small.txt")));
        assert_eq!(res, 80);
    }

    #[test]
    fn example2_medium() {
        let res = two(&parse(include_str!("test_medium.txt")));
        assert_eq!(res, 436);
    }

    #[test]
    fn example2_e() {
        let res = two(&parse(include_str!("test_e.txt")));
        assert_eq!(res, 236);
    }

    #[test]
    fn example2_abba() {
        let res = two(&parse(include_str!("test_abba.txt")));
        assert_eq!(res, 368);
    }

    #[test]
    fn example2() {
        let res = two(&parse(include_str!("test.txt")));
        assert_eq!(res, 1206);
    }
}
