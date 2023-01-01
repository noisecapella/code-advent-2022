use std::cmp::{max, min};
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

fn find_best_path(valves_with_flow: &Vec<Valve>, distances: &HashMap<(CoordType, CoordType), i64>, start: CoordType) -> i64 {
    struct State {
        path: Vec<CoordType>,
        minutes: i64,
        flow: i64
    }

    let mut old_moves = vec![State {
        path: vec![start],
        minutes: 0,
        flow: 0
    }];
    let mut solutions: Vec<i64> = vec![];

    println!("distances {} {:?}", distances.len(), distances);
    println!("valves {} {:?}", valves_with_flow.len(), valves_with_flow);
    println!("start {:?}", start);

    loop {
        if old_moves.is_empty() {
            let ret = solutions.iter().max().unwrap();
            return *ret;
        }
        println!("old_moves {}", old_moves.len());
        let new_moves: Vec<State> = old_moves.iter().filter_map(|old_state| {
            if old_state.minutes >= 30 {
                return None
            }

            let mut possibilities: Vec<_> = valves_with_flow.iter().filter_map(|valve| {
                if old_state.path.contains(&valve.name) {
                    return None;
                }

                let minutes = old_state.minutes + distances[&(*old_state.path.last().unwrap(), valve.name)];
                let flow = ((30 - minutes) * valve.flow_rate) + old_state.flow;
                let mut new_move_path = old_state.path.clone();
                new_move_path.push(valve.name);
                Some(State {
                    path: new_move_path,
                    minutes,
                    flow
                })
            }).collect();
            possibilities.push(
                State {
                    path: old_state.path.clone(),
                    minutes: 30,
                    flow: old_state.flow
                }
            );
            Some(possibilities)
        }).flatten().collect();

        for new_move in new_moves.iter() {
            if new_move.minutes >= 30 {
                solutions.push(new_move.flow);
            }
        }

        old_moves = new_moves;
    }
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

    let distances = calc_distances(&paths);

    let best_flow = find_best_path(&valves_with_flow, &distances, start);

    //println!("best path {:?} {}", best_path, best_path_flow);
    best_flow
}


fn find_best_path_elephant(valves_with_flow: &Vec<Valve>, distances: &HashMap<(CoordType, CoordType), i64>, start: CoordType) -> i64 {
    struct State {
        path: Vec<CoordType>,
        me_minutes: i64,
        me_current: CoordType,
        elephant_minutes: i64,
        elephant_current: CoordType,
        flow: i64
    }

    const TOTAL_MINUTES: i64 = 26;
    let mut old_moves: Vec<State> = vec![State {
        path: vec![start],
        me_minutes: 0,
        me_current: start,
        elephant_minutes: 0,
        elephant_current: start,
        flow: 0
    }];
    let mut max_flow = 0;

    println!("distances {} {:?}", distances.len(), distances);
    println!("valves {} {:?}", valves_with_flow.len(), valves_with_flow);
    println!("start {:?}", start);

    loop {
        if old_moves.is_empty() {
            return max_flow;
        }
        println!("old_moves {}", old_moves.len());
        let new_moves = old_moves.iter().filter_map(|old_state| {

            Some(valves_with_flow.iter().filter_map(|valve| {
                if old_state.me_minutes >= TOTAL_MINUTES {
                    return None;
                }
                if old_state.path.contains(&valve.name) {
                    return None;
                }

                let minutes = old_state.me_minutes + distances[&(old_state.me_current, valve.name)];
                let new_flow = (max((TOTAL_MINUTES - minutes), 0) * valve.flow_rate);
                if new_flow == 0 {
                    return None
                }
                let flow = new_flow + old_state.flow;
                let mut new_move_path = old_state.path.clone();
                new_move_path.push(valve.name);

                Some(State {
                    path: new_move_path,
                    me_minutes: minutes,
                    me_current: valve.name,
                    elephant_minutes: old_state.elephant_minutes,
                    elephant_current: old_state.elephant_current,
                    flow: flow
                })
            }).chain(valves_with_flow.iter().filter_map(|valve| {
                if old_state.elephant_minutes >= TOTAL_MINUTES {
                    return None;
                }
                if old_state.path.contains(&valve.name) {
                    return None;
                }

                let minutes = old_state.elephant_minutes + distances[&(old_state.elephant_current, valve.name)];
                let new_flow = (max((TOTAL_MINUTES - minutes), 0) * valve.flow_rate);
                if new_flow == 0 {
                    return None
                }
                let flow = new_flow + old_state.flow;
                let mut new_move_path = old_state.path.clone();
                new_move_path.push(valve.name);
                Some(State {
                    path: new_move_path,
                    me_minutes: old_state.me_minutes,
                    me_current: old_state.me_current,
                    elephant_minutes: minutes,
                    elephant_current: valve.name,
                    flow: flow
                })

            })))
        }).flatten();

        /*
        for new_move in new_moves.iter() {
            if new_move.me_minutes >= TOTAL_MINUTES && new_move.elephant_minutes >= TOTAL_MINUTES {
                if new_move.flow > max_flow {
                    //max_flow = new_move.flow;
                }
            }
        }*/

        old_moves = new_moves.into_iter().filter(|new_move| {
            if new_move.flow > max_flow {
                max_flow = new_move.flow;
            }
            if (new_move.me_minutes >= TOTAL_MINUTES && new_move.elephant_minutes >= TOTAL_MINUTES) {
                return false;
            }


            let max_remaining_flow_rate = {
                valves_with_flow.iter().filter_map(|other_valve| {
                    if new_move.path.contains(&other_valve.name) {
                        None
                    } else {
                        Some(
                            max(max(
                                (TOTAL_MINUTES - new_move.me_minutes - distances[&(new_move.me_current, other_valve.name)]) * other_valve.flow_rate,
                                (TOTAL_MINUTES - new_move.elephant_minutes - distances[&(new_move.elephant_current, other_valve.name)]) * other_valve.flow_rate,
                            ), 0)
                        )
                    }
                }).max()
            };

            match max_remaining_flow_rate {
                Some(_max_remaining_flow) => {
                    if max_flow > (_max_remaining_flow + new_move.flow) {
                        return false;
                    }
                }
                _ => {}
            }

            return true;
        }
        ).collect();
    }
}

pub fn part2(file_path: &str) -> i64 {
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

    let distances = calc_distances(&paths);

    let best_flow = find_best_path_elephant(&valves_with_flow, &distances, start);

    //println!("best path {:?} {}", best_path, best_path_flow);
    best_flow
}