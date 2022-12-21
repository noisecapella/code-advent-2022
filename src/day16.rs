use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use clap::value_parser;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use crate::common::{ get_trimmed_lines};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct CoordType {
    letter1: u8,
    letter2: u8,
    is_valve: bool
}

impl fmt::Debug for CoordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.letter1.escape_ascii(), self.letter2.escape_ascii(), if self.is_valve { "+" } else { "" })
    }
}

type CostType = f64;

#[derive(Debug)]
pub struct Valve {
    name: CoordType,
    flow_rate: i64,
    next_valves: Vec<CoordType>
}

fn to_valve_key(key: &str) -> CoordType {
    let b: Vec<u8> = key.bytes().collect();
    if b.len() == 2 {
        CoordType { letter1: b[0], letter2: b[1], is_valve: false }
    } else {
        CoordType { letter1: b[0], letter2: b[1], is_valve: true }
    }
}

fn read_valves(file_path: &str) -> Vec<Valve> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
    }

    let lines = get_trimmed_lines(file_path);
    lines.iter().map(|line| {
        match RE.captures(line) {
            Some(cap) => {
                Valve {
                    name: to_valve_key(cap.get(1).unwrap().as_str()),
                    flow_rate: cap.get(2).unwrap().as_str().parse().unwrap(),
                    next_valves: cap.get(3).unwrap().as_str().split(", ").map(|s| to_valve_key(s)).collect()
                }
            },
            None => {
                panic!("unknown line {}", line)
            }
        }
    }).collect()
}


fn reconstruct_path(came_from: &HashMap<CoordType, CoordType>, current: CoordType) -> VecDeque<CoordType> {
    let mut total_path: VecDeque<CoordType> = VecDeque::new();
    let mut current_mut = current;
    while came_from.contains_key(&current_mut) {
        current_mut = came_from[&current_mut];
        total_path.push_front(current_mut);
    }
    return total_path;
}

pub fn dijkstra(valves_map: &HashMap<CoordType, Valve>, source: CoordType) -> HashMap<CoordType, CoordType> {
    let valve_keys: Vec<CoordType> = valves_map.keys().map(|key| [*key, CoordType {
        letter1: key.letter1,
        letter2: key.letter2,
        is_valve: true,
    }]).flatten().collect();
    let mut dist: HashMap<CoordType, f64> = valve_keys.iter().map(|key| (*key, f64::INFINITY)).collect();
    let mut prev: HashMap<CoordType, CoordType> = HashMap::new();
    let mut queue: HashSet<CoordType> = valve_keys.iter().map(|key| *key).collect();

    let next_valves: HashMap<CoordType, (Vec<(CoordType, f64)>)> = valve_keys.iter().map(|key| {
        if !key.is_valve {
            let mut next_valves: Vec<(CoordType, f64)> = Vec::new();
            let valve = &valves_map[key];
            for _next in valve.next_valves.iter() {
                next_valves.push((*_next, 1f64));
            }
            next_valves.push((CoordType {
                letter1: key.letter1,
                letter2: key.letter2,
                is_valve: true
            }, 0.5f64));
            (*key, next_valves)
        } else {
            let mut next_valves: Vec<(CoordType, f64)> = Vec::new();
            let valve = &valves_map[&CoordType {
                letter1: key.letter1,
                letter2: key.letter2,
                is_valve: false
            }];
            for _next in valve.next_valves.iter() {
                next_valves.push((*_next, 1.5f64));
            }
            (*key, next_valves)
        }
    }).collect();

    dist.insert(source,0f64);

    while !queue.is_empty() {
        let u = {
            let mut lowest: Option<(CoordType, f64)> = None;

            for vertex in queue.iter() {
                match lowest {
                    None => {
                        lowest = Some((*vertex, dist[&vertex]));
                    },
                    Some(_lowest) => {
                        if dist[&vertex] < _lowest.1 {
                            lowest = Some((*vertex, dist[&vertex]));
                        }
                    }
                }
            }

            lowest.unwrap().0
        };

        queue.remove(&u);

        for (v, cost) in next_valves[&u].iter() {
            if !queue.contains(&v) {
                continue;
            }

            let alt = dist[&u] + cost;
            if alt < dist[&v] {
                dist.insert(*v, alt);
                prev.insert(*v, u);
            }
        }
    }
    //println!("prev {:?}", prev);
    //println!("dist {:?}", dist);

    /*
    let path = {
        let mut current = source;
        let mut path: VecDeque<CoordType> = [current].iter().map(|item| *item).collect();

        loop {
            println!("{:?}", current);
            match prev.get(&current) {
                Some(_prev) => {
                    path.push_back(*_prev);
                    current = *_prev;
                },
                None => {
                    break;
                }
            }
        }

        path
    };
*/
    prev
}


fn make_path(valves_with_flow: &Vec<CoordType>, paths: &HashMap<CoordType, HashMap<CoordType, CoordType>>, start: CoordType) -> Vec<CoordType> {
    let mut path= Vec::new();
    let mut prev = start;
    for valve in valves_with_flow.iter() {
        for piece in reconstruct_path(&paths[&prev], *valve) {
            path.push(piece);
        }
        prev = *valve;
    }
    path.push(prev);
    path.remove(0);
    path
}

fn calc_flow(path: &Vec<CoordType>, valves_map: &HashMap<CoordType, Valve>) -> i64 {
    //println!("{:?}", path);

    let mut cum_flow = 0;
    let mut flow = 0;
    for minute in 0..30 {
        cum_flow += flow;

        match path.get(minute) {
            None => {
                //println!("Minute {}: nothing to do", minute);
            },
            Some(item) => {
                if item.is_valve {
                    let valve = &valves_map[&CoordType {
                        letter1: item.letter1,
                        letter2: item.letter2,
                        is_valve: false,
                    }];

                    flow += valve.flow_rate;
                    //println!("Minute {}: opened valve {:?} to release {}", minute, item, valve.flow_rate);
                } else {
                    //println!("Minute {}: moved to item {:?}", minute, item);
                }
            }
        }

        //println!("cumulative flow is {}, flow is {}", cum_flow, flow);
    }

    cum_flow
}

/*fn backtrack(valves_map: &HashMap<CoordType, Valve>, valves_left: &Vec<&Valve>, path_distances: &HashMap<(CoordType, CoordType), u64>, current_flow: i64, minutes: u64) -> i64 {

    let max_possible_flow = valves_left.iter().filter_map(|next_valve| {
        let valves_left_next = valves_left.iter().filter_map(|_valve| {
            if _valve.name != next_valve.name {
                Some(*_valve)
            } else {
                None
            }
        }).collect();

        if minutes + 1 >= 30 {
            return None
        }

        let result = backtrack(&valves_map, &valves_left_next, &path_distances, current_flow + next_valve.flow_rate, minutes + 1);

        Some(result)
    }).max().unwrap();

    max_possible_flow
}*/

fn find_best_path(valves_with_flow: &Vec<Valve>, distances: &HashMap<(CoordType, CoordType), i64>, start: CoordType) -> (Vec<CoordType>, i64) {
    let mut ret_coords = Vec::new();
    let mut ret_flow = 0;
    let mut minutes = 0;

    let mut prev = start;
    while ret_coords.len() != valves_with_flow.len() {
        let result = valves_with_flow.iter().filter_map(|valve| {
            if ret_coords.contains(&valve.name) {
                return None;
            }

            let distance = distances.get(&(prev, valve.name)).unwrap();
            println!("distance {:?} {:?} = {}", prev, valve.name, distance);
            if distance + minutes > 30 {
                None
            } else {
                println!("{:?} to {:?} with flow rate = {}, distance = {}, {} minutes left, has total lifetime flow {}", prev, valve.name, valve.flow_rate, distance, 30 - minutes, (30 - minutes - distance) * valve.flow_rate);
                Some(((30 - minutes - distance) * valve.flow_rate, distance, valve.name))
            }
        }).max_by_key(|tup| tup.0);

        match result {
            None => {
                panic!("what?")
            },
            Some((best_flow, best_distance, best_coord)) => {
                ret_coords.push(best_coord);
                ret_flow += best_flow;
                minutes += best_distance - 1;
                prev = best_coord;
            }
        }
    }

    (ret_coords, ret_flow)
}

fn update_is_valve(coord: CoordType, is_valve: bool) -> CoordType {
    CoordType {
        letter1: coord.letter1,
        letter2: coord.letter2,
        is_valve
    }
}

fn calc_distances(paths: &HashMap<CoordType, HashMap<CoordType, CoordType>>) -> HashMap<(CoordType, CoordType), i64> {
    let mut distances: HashMap<(CoordType, CoordType), i64> = HashMap::new();

    for prev_valve in paths.keys() {
        for next_valve in paths.keys() {
            if next_valve == prev_valve {
                continue;
            }
            let path = reconstruct_path(&paths[&prev_valve], *next_valve);
            distances.insert((*prev_valve, *next_valve), path.len() as i64);
        }
    }

    distances
}

pub fn part1(file_path: &str) -> i64 {
    let valves = read_valves(file_path);
    // let num_valves = valves.iter().filter(|valve| valve.flow_rate > 0).count();
    let valves_map: HashMap<CoordType, Valve> = valves.into_iter().map(|valve| (valve.name, valve)).collect();

    let start = to_valve_key("AA");
    //let result = dijkstra(&valves_map, start);
    let mut paths: HashMap<CoordType, HashMap<CoordType, CoordType>> = HashMap::new();

    let valves_with_flow: Vec<Valve> = valves_map.values().filter_map(|valve| {
        if valve.flow_rate > 0 {
            Some(Valve {
                name: update_is_valve(valve.name, true),
                flow_rate: valve.flow_rate,
                next_valves: valve.next_valves.clone(),
            })
        } else {
            None
        }
    }).collect();

    for valve_key in valves_map.keys() {
        paths.insert(*valve_key, dijkstra(&valves_map, *valve_key));
        let coord_type = CoordType {
            letter1: valve_key.letter1,
            letter2: valve_key.letter2,
            is_valve: true
        };
        paths.insert(coord_type, dijkstra(&valves_map, coord_type));
    }

    /*
    let (best_path, best_path_flow) = valves_with_flow.iter().permutations(valves_with_flow.len()).map(|permutation| {
        let keys: Vec<CoordType> = permutation.iter().map(|x| **x).collect();
        let path = make_path(&keys, &paths, start);
        (path.clone(), calc_flow(&path, &valves_map))
    }).max_by_key(|tup| tup.1).unwrap();*/
    /*
    let best_path_flow = backtrack(
        &valves_map,
        &valves_with_flow,
        &paths,
        0,
        30
    );*/

    let distances = calc_distances(&paths);

    let (best_coords, best_flow) = find_best_path(&valves_with_flow, &distances, start);

    //println!("best path {:?} {}", best_path, best_path_flow);
    best_flow
}

pub fn part2(file_path: &str) -> i64 {
    0
}