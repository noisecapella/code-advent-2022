use std::cmp::max;
use std::collections::HashSet;
use std::fs::read;
use std::iter::{empty, once, zip};
use std::path::Iter;
use lazy_static::lazy_static;
use regex::Regex;
use crate::common::get_trimmed_lines;
use std::thread;
use std::thread::JoinHandle;
use itertools::chain;

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

#[derive(Copy, Debug, Clone, Eq, PartialEq, Hash)]
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

fn _calc_blueprint(blueprint: Blueprint, start: &Resources, total_minutes: u64) -> (Blueprint, u64) {
    let mut minute = 0;
    let mut choices = HashSet::from([*start]);
    loop {
        println!("blueprint: {}, minute: {}, choices: {}", blueprint.number, minute, choices.len());
        if minute >= total_minutes {
            return (blueprint, choices.iter().max_by_key(|resources| resources.num_geode).unwrap().num_geode);
        }

        let mut new_choices = HashSet::new();

        for resources in choices.iter() {
            if resources.num_ore >= blueprint.geode.ore && resources.num_obsidian >= blueprint.geode.obsidian {
                new_choices.insert(Resources {
                    num_ore_robots: resources.num_ore_robots,
                    num_clay_robots: resources.num_clay_robots,
                    num_obsidian_robots: resources.num_obsidian_robots,
                    num_geode_robots: resources.num_geode_robots + 1,
                    num_ore: resources.num_ore + resources.num_ore_robots - blueprint.geode.ore,
                    num_clay: resources.num_clay + resources.num_clay_robots,
                    num_obsidian: resources.num_obsidian + resources.num_obsidian_robots - blueprint.geode.obsidian,
                    num_geode: resources.num_geode + resources.num_geode_robots,
                });
                continue;
            }

            if resources.num_ore >= blueprint.obsidian.ore && resources.num_clay >= blueprint.obsidian.clay &&
                resources.num_obsidian_robots < blueprint.geode.obsidian {
                new_choices.insert(Resources {
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

            if resources.num_ore >= blueprint.clay.ore && resources.num_clay_robots < blueprint.obsidian.clay {
                new_choices.insert(Resources {
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

            if resources.num_ore >= blueprint.ore.ore && resources.num_ore_robots < max(max(blueprint.obsidian.ore, blueprint.geode.ore), blueprint.clay.ore) {
                new_choices.insert(Resources {
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

            new_choices.insert(Resources {
                num_ore_robots: resources.num_ore_robots,
                num_clay_robots: resources.num_clay_robots,
                num_obsidian_robots: resources.num_obsidian_robots,
                num_geode_robots: resources.num_geode_robots,
                num_ore: resources.num_ore + resources.num_ore_robots,
                num_clay: resources.num_clay + resources.num_clay_robots,
                num_obsidian: resources.num_obsidian + resources.num_obsidian_robots,
                num_geode: resources.num_geode + resources.num_geode_robots,
            });
        }

        /*
        if choices.is_empty() {
            choices = new_choices;
        } else {
            let current_max_geode_resource = *choices.iter().max_by_key(|c| (c.num_geode, c.num_geode_robots)).unwrap();
            let current_max_geode = current_max_geode_resource.num_geode + ((total_minutes - minute) * current_max_geode_resource.num_geode_robots);

            choices = new_choices.into_iter().filter(|resources| {
                let mut _resources = *resources;
                let mut possible_total = resources.num_geode;
                for future_minute in (minute + 1)..total_minutes {
                    if _resources.num_ore >= blueprint.geode.ore && _resources.num_obsidian >= blueprint.geode.obsidian {
                        possible_total += (resources.num_geode_robots + (future_minute - minute)) * (total_minutes - future_minute);
                    } else {
                        _resources.num_ore += blueprint.geode.ore;
                        _resources.num_obsidian += blueprint.geode.obsidian;
                    }
                }
                possible_total >= current_max_geode
            }).collect();
        }*/
        choices = new_choices;
        minute += 1;
    }
}

fn calc_blueprint(blueprint: Blueprint, total_minutes: u64) -> (Blueprint, u64) {
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

    _calc_blueprint(blueprint, &resources, total_minutes)
}

pub fn part1(file_path: &str) -> i64 {
    let blueprints: Vec<_> = read_blueprints(file_path);

    let threads: Vec<JoinHandle<(Blueprint, u64)>> = blueprints.into_iter().map(|blueprint| {
        thread::spawn(move || {
            calc_blueprint(blueprint, 24)
        })
    }).collect();

    let results: Vec<(Blueprint, u64)> = threads.into_iter().map(|handle| {
        handle.join().unwrap()
    }).collect();

    //let results: Vec<_> = blueprints.iter().map(|blueprint| calc_blueprint(blueprint)).collect();

    println!("results {:?}", results);
    results.iter().map(|(blueprint, result)| {
        blueprint.number as i64 * *result as i64
    }).sum()
}

pub fn part2(file_path: &str) -> i64 {
    let blueprints: Vec<_> = read_blueprints(file_path).iter().take(3).map(|x| *x).collect();

    let threads: Vec<JoinHandle<(Blueprint, u64)>> = blueprints.into_iter().map(|blueprint| {
        thread::spawn(move || {
            calc_blueprint(blueprint, 32)
        })
    }).collect();

    let results: Vec<(Blueprint, u64)> = threads.into_iter().map(|handle| {
        handle.join().unwrap()
    }).collect();

    //let results: Vec<_> = blueprints.iter().map(|blueprint| calc_blueprint(blueprint)).collect();

    println!("results {:?}", results);
    results.iter().map(|(blueprint, result)| {
        *result as i64
    }).product()
}