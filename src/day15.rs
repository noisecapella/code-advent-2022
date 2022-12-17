use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use crate::common::get_trimmed_lines;
use std::cmp::max;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    bx: i64,
    by: i64,
}


fn read_sensors(file_path: &str) -> Vec<Sensor> {
    let lines = get_trimmed_lines(&file_path);

    lazy_static! {
        static ref RE: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    }

    lines.iter().map(|line| {
        match RE.captures(line) {
            Some(cap) => {
                Sensor {
                    x: cap.get(1).unwrap().as_str().parse().unwrap(),
                    y: cap.get(2).unwrap().as_str().parse().unwrap(),
                    bx: cap.get(3).unwrap().as_str().parse().unwrap(),
                    by: cap.get(4).unwrap().as_str().parse().unwrap(),
                }
            },
            None => {
                panic!("unexpected line")
            }
        }
    }).collect()
}

#[test]
fn test_sort_filter_exclusions() {
    let input: Vec<(i64, i64)> = vec![(12, 13), (2, 15), (2, 3), (-2, 3), (16, 25), (14, 19)];
    // sorted: (-2, 3), (2, 3), (2, 15), (12, 13), (14, 19), (16, 25)
    let sorted: Vec<&(i64, i64)> = input.iter().sorted().collect();
    println!("sorted {:?}", sorted);
    assert_eq!(
        sort_filter_exclusions(
            &input,
        ),
        vec![
            (-2, 3), (3, 3), (3, 15), (15, 15), (15, 19), (19, 25)
        ]
    );
}

#[test]
fn test_combine_exclusions() {
    let input: Vec<(i64, i64)> = vec![(-2, 3), (3, 3), (3, 15), (15, 15), (15, 19), (19, 25)];
    assert_eq!(combine_exclusions(&input), vec![(-2, 25)]);
    assert_eq!(combine_exclusions(&vec![(1i64,3i64), (3i64, 5i64), (6i64, 9i64)]), vec![(1,5), (6,9)]);
}

fn sort_filter_exclusions(exclusions: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut copy: Vec<(i64, i64)> = exclusions.clone();
    copy.sort();

    let mut prev = None;
    let mut filtered: Vec<(i64, i64)> = copy.iter().filter_map(|pair| {
        match prev {
            None => {
                prev = Some(*pair);
                Some(*pair)
            },
            Some(mut _prev) => {
                let result = (max(_prev.1, pair.0), max(_prev.1, pair.1));
                prev = Some(result);
                Some(result)
            }
        }
    }).collect();
    filtered.sort();
    filtered
}

fn combine_exclusions(exclusions: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut combined = Vec::new();
    for current in exclusions.iter() {
        let prev: Option<(i64, i64)> = combined.last().cloned();
        match prev {
            None => {
                combined.push(*current);
            },
            Some(_prev) => {
                if _prev.1 == current.0 {
                    combined.pop();
                    combined.push((_prev.0, current.1));
                } else {
                    combined.push(*current);
                }
            }
        }
    }

    combined

}

fn calc_exclusions(sensors: &Vec<Sensor>, row: i64) -> Vec<(i64, i64)> {
    let exclusions: Vec<(i64, i64)> = sensors.iter().filter_map(|sensor| {
        let distance = (sensor.x - sensor.bx).abs() + (sensor.y - sensor.by).abs();
        if (row - sensor.y).abs() > distance {
            None
        } else {
            let dx = (distance - (row - sensor.y).abs());
            Some((sensor.x - dx, sensor.x + dx + 1))
        }
    }).collect();

    let sorted_filter_exclusions = sort_filter_exclusions(&exclusions);

    combine_exclusions(&sorted_filter_exclusions)
}

fn count_exclusions(sensors: &Vec<Sensor>, row: i64) -> i64 {
    let sorted_exclusions = calc_exclusions(sensors, row);
    println!("{:?}", sorted_exclusions);
    let sum: i64 = sorted_exclusions.iter().map(|exclusion| {
        exclusion.1 - exclusion.0
    }).sum();
    sum - sensors.iter().filter_map(|sensor| {
        // println!("{:?} {}", sensor, row);
        if sensor.y == row && !contains_point(&sorted_exclusions, sensor.x).is_none() {
            Some((sensor.x, sensor.y))
        } else if sensor.by == row && !contains_point(&sorted_exclusions, sensor.bx).is_none() {
            Some((sensor.bx, sensor.by))
        } else {
            None
        }
    }).unique().count() as i64
}

fn contains_point(exclusions: &Vec<(i64, i64)>, col: i64) -> Option<usize> {
    let key = exclusions.binary_search_by_key(&col, |&(a, b)| a);

    match key {
        Ok(idx) => {
            Some(idx)
        },
        Err(idx) => {
            if idx == 0 {
                None
            } else {
                let prev = exclusions[idx - 1];
                if col < prev.1 {
                    Some(idx - 1)
                } else {
                    None
                }
            }
        }
    }
}

pub fn part1(file_path: &str) -> i64 {
    let sensors = read_sensors(file_path);

    //count_exclusions(&sensors, 10)
    count_exclusions(&sensors, 2000000)
}

pub fn part2(file_path: &str) -> i64 {
    let sensors = read_sensors(file_path);

    let bound = 4000000; //20;
    for y in 0..bound {
        let exclusions = calc_exclusions(&sensors, y);

        let mut x = 0;
        while x < bound {
            //println!("x {}", x);
            match contains_point(&exclusions, x) {
                Some(idx) => {
                    //println!("idx {} {:?}", idx, exclusions.len());
                    x = exclusions[idx].1;
                },
                None => {
                    println!("found {} {}", x, y);
                    return x*bound + y;
                }
            }
        }
    }

    0
    //count_exclusions(&sensors, 10)
    //count_exclusions(&sensors, 2000000)
}