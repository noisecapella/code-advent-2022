use std::fs::read;
use lazy_static::lazy_static;
use regex::Regex;
use crate::common::get_trimmed_lines;
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct OreRobot {
    ore: u64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct ClayRobot {
    ore: u64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct ObsidianRobot {
    ore: u64,
    clay: u64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct GeodeRobot {
    ore: u64,
    obsidian: u64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Blueprint {
    number: u64,
    ore: OreRobot,
    clay: ClayRobot,
    obsidian: ObsidianRobot,
    geode: GeodeRobot,
}

lazy_static! {
    static ref RE: Regex = Regex::new(
        concat!(
            r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. ",
            r"Each obsidian robot costs (\d+) ore and (\d+) clay. ",
            r"Each geode robot costs (\d+) ore and (\d+) obsidian."
        )
    ).unwrap();
}

fn read_blueprints(file_path: &str) -> Vec<Blueprint> {
    let lines = get_trimmed_lines(file_path);
    lines.iter().map(|line| {
        match RE.captures(line) {
            Some(cap) => {
                Blueprint {
                    number: cap.get(1).unwrap().as_str().parse().unwrap(),
                    ore: OreRobot { ore: cap.get(2).unwrap().as_str().parse().unwrap() },
                    clay: ClayRobot { ore: cap.get(3).unwrap().as_str().parse().unwrap() },
                    obsidian: ObsidianRobot {
                        ore: cap.get(4).unwrap().as_str().parse().unwrap(),
                        clay: cap.get(5).unwrap().as_str().parse().unwrap(),
                    },
                    geode: GeodeRobot {
                        ore: cap.get(6).unwrap().as_str().parse().unwrap(),
                        obsidian: cap.get(7).unwrap().as_str().parse().unwrap(),
                    }
                }
            },
            None => {
                panic!("unexpected regex mismatch")
            }
        }
    }).collect()
}

struct Resources {
    num_ore_robots: u64,
    num_clay_robots: u64,
    num_obsidian_robots: u64,
    num_geode_robots: u64,

    num_ore: u64,
    num_clay: u64,
    num_obsidian: u64,
    num_geode: u64,
}

fn _calc_blueprint(blueprint: &Blueprint, resources: &Resources, minute: u64) -> u64 {
    if minute == 24 {
        return resources.num_geode;
    }

    let mut choices = Vec::new();
    if resources.num_ore >= blueprint.geode.ore && resources.num_obsidian >= blueprint.geode.obsidian {
        choices.push(Resources {
            num_ore_robots: resources.num_ore_robots,
            num_clay_robots: resources.num_clay_robots,
            num_obsidian_robots: resources.num_obsidian_robots,
            num_geode_robots: resources.num_geode_robots + 1,
            num_ore: resources.num_ore + resources.num_ore_robots - blueprint.geode.ore,
            num_clay: resources.num_clay + resources.num_clay_robots,
            num_obsidian: resources.num_obsidian + resources.num_obsidian_robots - blueprint.geode.obsidian,
            num_geode: resources.num_geode + resources.num_geode_robots,
        });
    }

    if resources.num_ore >= blueprint.obsidian.ore && resources.num_clay >= blueprint.obsidian.clay {
        choices.push(Resources {
            num_ore_robots: resources.num_ore_robots,
            num_clay_robots: resources.num_clay_robots,
            num_obsidian_robots: resources.num_obsidian_robots + 1,
            num_geode_robots: resources.num_geode_robots,
            num_ore: resources.num_ore + resources.num_ore_robots - blueprint.obsidian.ore,
            num_clay: resources.num_clay + resources.num_clay_robots - blueprint.obsidian.clay,
            num_obsidian: resources.num_obsidian + resources.num_obsidian_robots,
            num_geode: resources.num_geode + resources.num_geode_robots,
        });
    }

    if resources.num_ore >= blueprint.clay.ore {
        choices.push(Resources {
            num_ore_robots: resources.num_ore_robots,
            num_clay_robots: resources.num_clay_robots + 1,
            num_obsidian_robots: resources.num_obsidian_robots,
            num_geode_robots: resources.num_geode_robots,
            num_ore: resources.num_ore + resources.num_ore_robots - blueprint.clay.ore,
            num_clay: resources.num_clay + resources.num_clay_robots,
            num_obsidian: resources.num_obsidian + resources.num_obsidian_robots,
            num_geode: resources.num_geode + resources.num_geode_robots,
        });
    }

    if resources.num_ore >= blueprint.ore.ore {
        choices.push(Resources {
            num_ore_robots: resources.num_ore_robots + 1,
            num_clay_robots: resources.num_clay_robots,
            num_obsidian_robots: resources.num_obsidian_robots,
            num_geode_robots: resources.num_geode_robots,
            num_ore: resources.num_ore + resources.num_ore_robots - blueprint.ore.ore,
            num_clay: resources.num_clay + resources.num_clay_robots,
            num_obsidian: resources.num_obsidian + resources.num_obsidian_robots,
            num_geode: resources.num_geode + resources.num_geode_robots,
        });
    }

    choices.push(Resources {
        num_ore_robots: resources.num_ore_robots,
        num_clay_robots: resources.num_clay_robots,
        num_obsidian_robots: resources.num_obsidian_robots,
        num_geode_robots: resources.num_geode_robots,
        num_ore: resources.num_ore + resources.num_ore_robots,
        num_clay: resources.num_clay + resources.num_clay_robots,
        num_obsidian: resources.num_obsidian + resources.num_obsidian_robots,
        num_geode: resources.num_geode + resources.num_geode_robots,
    });

    choices.iter().map(|choice| {
        _calc_blueprint(&blueprint, choice, minute + 1)
    }).max().unwrap()
}

fn calc_blueprint(blueprint: &Blueprint) -> u64 {
    let resources = Resources {
        num_ore_robots: 1,
        num_clay_robots: 0,
        num_obsidian_robots: 0,
        num_geode_robots: 0,
        num_ore: 0,
        num_clay: 0,
        num_obsidian: 0,
        num_geode: 0,
    };

    _calc_blueprint(blueprint, &resources, 0)
}

pub fn part1(file_path: &str) -> i64 {
    let blueprints = read_blueprints(file_path);

    let threads: Vec<JoinHandle<u64>> = blueprints.into_iter().map(|blueprint| {
        thread::spawn(move || {
            calc_blueprint(&blueprint)
        })
    }).collect();

    let results: Vec<u64> = threads.into_iter().map(|handle| {
        handle.join().unwrap()
    }).collect();

    println!("results {:?}", results);
    0
}

pub fn part2(file_path: &str) -> i64 {
    0
}