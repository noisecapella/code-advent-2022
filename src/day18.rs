use std::collections::HashSet;
use crate::common::get_trimmed_lines;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Adjustment {
    x: i64,
    y: i64,
    z: i64,
}

fn read_tuples(file_path: &str) -> Vec<Coord> {
    get_trimmed_lines(file_path).iter().map(|line| {
        let mut split = line.split(",");
        let a: i64 = split.next().unwrap().parse().unwrap();
        let b: i64 = split.next().unwrap().parse().unwrap();
        let c: i64 = split.next().unwrap().parse().unwrap();
        Coord { x:a, y:b, z:c }
    }).collect()
}

pub fn part1(file_path: &str) -> i64 {
    let tuples = read_tuples(file_path);

    let s: HashSet<Coord> = HashSet::from_iter(tuples.iter().map(|t| *t));

    let mut count = 0;
    for tup in s.iter() {
        for adjustment in [
            Adjustment { x:-1, y:0, z:0 },
            Adjustment {x:1, y:0, z:0},
            Adjustment {x:0, y:-1, z:0},
            Adjustment {x:0, y:1, z:0},
            Adjustment {x:0, y:0, z:-1},
            Adjustment {x:0, y:0, z:1}
        ] {
            if !s.contains(&add(*tup, adjustment)) {
                count += 1;
            }
        }
    }

    count
}

fn add(t1: Coord, t2: Adjustment) -> Coord {
    Coord { x: t1.x + t2.x, y: t1.y + t2.y, z: t1.z + t2.z }
}

fn _out_of_bounds(coord: Coord, bounds: &[(i64, i64, i64); 2]) -> bool {
    coord.x < bounds[0].0 ||
    coord.y < bounds[0].1 ||
    coord.z < bounds[0].2 ||
    coord.x >= bounds[1].0 ||
    coord.y >= bounds[1].1 ||
    coord.z >= bounds[1].2
}

fn _touching_outside(coords_outside: &mut HashSet<Coord>, cubes: &HashSet<Coord>, coord: Coord, bounds: &[(i64, i64, i64); 2], search_space: &mut HashSet<Coord>) -> bool {
    search_space.insert(coord);
    for adjustment in [
        Adjustment { x:-1, y:0, z:0 },
        Adjustment {x:1, y:0, z:0},
        Adjustment {x:0, y:-1, z:0},
        Adjustment {x:0, y:1, z:0},
        Adjustment {x:0, y:0, z:-1},
        Adjustment {x:0, y:0, z:1}
    ] {
        let adjusted = add(coord, adjustment);

        if search_space.contains(&adjusted) {
            // already looked here
            continue;
        }
        if _out_of_bounds(adjusted, bounds) {
            return true;
        }

        search_space.insert(adjusted);

        if cubes.contains(&adjusted) {
            continue;
        }

        if coords_outside.contains(&adjusted) {
            return true;
        }

        if _touching_outside(coords_outside, cubes, adjusted, bounds, search_space) {
            coords_outside.insert(adjusted);
            return true;
        }
    }

    false
}

fn touching_outside(coords_outside: &mut HashSet<Coord>, cubes: &HashSet<Coord>, coord: Coord, bounds: &[(i64, i64, i64); 2]) -> bool {
    _touching_outside(coords_outside, &cubes, coord, bounds, &mut HashSet::new())
}

pub fn part2(file_path: &str) -> i64 {
    let tuples = read_tuples(file_path);

    let cubes: HashSet<Coord> = HashSet::from_iter(tuples.iter().map(|t| *t));

    let bounds = [
        (
            cubes.iter().map(|triple| triple.x).min().unwrap(),
            cubes.iter().map(|triple| triple.y).min().unwrap(),
            cubes.iter().map(|triple| triple.z).min().unwrap(),
        ), (
            cubes.iter().map(|triple| triple.x).max().unwrap() + 1,
            cubes.iter().map(|triple| triple.y).max().unwrap() + 1,
            cubes.iter().map(|triple| triple.z).max().unwrap() + 1,
        )
    ];

    // non-cube connecting to outside
    let mut coords_outside: HashSet<Coord> = HashSet::new();

    let mut count = 0;
    for tup in cubes.iter() {
        for adjustment in [
            Adjustment { x:-1, y:0, z:0 },
            Adjustment {x:1, y:0, z:0},
            Adjustment {x:0, y:-1, z:0},
            Adjustment {x:0, y:1, z:0},
            Adjustment {x:0, y:0, z:-1},
            Adjustment {x:0, y:0, z:1}
        ] {
            let adjusted = add(*tup, adjustment);
            if !cubes.contains(&adjusted) {
                if touching_outside(&mut coords_outside, &cubes, adjusted, &bounds) {
                    count += 1;
                }
            }
        }
    }

    // 686 is too low
    count
}
