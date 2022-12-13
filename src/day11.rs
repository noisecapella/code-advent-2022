use std::collections::VecDeque;
use std::fmt;
use crate::common::get_trimmed_lines;
use regex::Regex;
use lazy_static::lazy_static;
use fmt::Debug;

lazy_static! {
    static ref MONKEY_RE: Regex = Regex::new(r"Monkey (\d+):").unwrap();
    static ref STARTING_RE: Regex = Regex::new(r"Starting items: (.+)").unwrap();
    static ref OPERATION_RE: Regex = Regex::new(r"Operation: new = old ([+*]) (old|\d+)").unwrap();
    static ref TEST_RE: Regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
    static ref IF_RE: Regex = Regex::new(r"If (true|false): throw to monkey (\d+)").unwrap();
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Monkey {
    number: usize,
    items: VecDeque<usize>,
    operation: Operation,
    operand: Option<usize>,
    test: usize,
    when_true: usize,
    when_false: usize,
}

fn make_monkey_lists(lines: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut monkey_num: Option<usize> = None;
    let mut items: Option<VecDeque<usize>> = None;
    let mut operation: Option<Operation> = None;
    let mut operand: Option<Option<usize>> = None;
    let mut test:  Option<usize> = None;
    let mut when_true: Option<usize> = None;
    let mut when_false: Option<usize> = None;

    for line in lines {
        match MONKEY_RE.captures(line) {
            Some(cap) => {
                let _monkey_num: usize = cap.get(1).unwrap().as_str().parse().unwrap();

                if _monkey_num != 0 {
                    monkeys.push(Monkey {
                        number: monkey_num.unwrap(),
                        items: items.unwrap(),
                        operation: operation.unwrap(),
                        operand: operand.unwrap(),
                        test: test.unwrap(),
                        when_true: when_true.unwrap(),
                        when_false: when_false.unwrap(),
                    });

                    monkey_num = None;
                    items = None;
                    operation = None;
                    operand = None;
                    test = None;
                    when_true = None;
                    when_false = None;
                }

                if _monkey_num != monkeys.len() {
                    panic!("Unknown monkey {}", _monkey_num);
                }

                monkey_num = Some(_monkey_num);

                continue;
            },
            None => { }
        };

        match STARTING_RE.captures(line) {
            Some(cap) => {
                let _items: VecDeque<usize> = cap.get(1).unwrap().as_str().split(",").map(|piece| piece.trim().parse::<usize>().unwrap()).collect();
                items = Some(_items);
                continue;
            },
            None => {}
        }

        match OPERATION_RE.captures(line) {
            Some(cap) => {
                let _operation = cap.get(1).unwrap().as_str();
                operation = Some(match _operation {
                    "*" => Operation::Multiply,
                    "+" => Operation::Add,
                    _ => panic!("Unknown operation")
                });
                let _operand = cap.get(2).unwrap().as_str();
                if _operand == "old" {
                    operand = Some(None)
                } else {
                    operand = Some(Some(_operand.parse().unwrap()));
                }
                continue;
            },
            None => {}
        }

        match TEST_RE.captures(line) {
            Some(cap) => {
                let _divisible: usize = cap.get(1).unwrap().as_str().parse().unwrap();
                test = Some(_divisible);
                continue;
            },
            None => {}
        }

        match IF_RE.captures(line) {
            Some(cap) => {
                let true_or_false = cap.get(1).unwrap().as_str();
                let other_monkey = cap.get(2).unwrap().as_str().parse().unwrap();
                match true_or_false {
                    "true" => {
                        when_true = Some(other_monkey);
                    },
                    "false" => {
                        when_false = Some(other_monkey);
                    },
                    _ => {
                        panic!("unexpected true_or_false");
                    }
                }
                continue;
            },
            None => {}
        }

        panic!("Unexpected line");
    }

    monkeys.push(Monkey {
        number: monkey_num.unwrap(),
        items: items.unwrap(),
        operation: operation.unwrap(),
        operand: operand.unwrap(),
        test: test.unwrap(),
        when_true: when_true.unwrap(),
        when_false: when_false.unwrap(),
    });

    monkeys
}

fn calc_monkey_business(file_path: &str, div_3: bool, num_rounds: usize) -> usize {
    let lines = get_trimmed_lines(file_path);
    let mut monkeys = make_monkey_lists(&lines);

    let mut divisible = 1;
    for monkey_idx in 0..monkeys.len() {
        divisible *= monkeys[monkey_idx].test;
    }

    let mut activity: Vec<usize> = (0..monkeys.len()).map(|_| 0).collect();

    for round in 0..num_rounds {
        println!("{}", round);
        for monkey_idx in 0..monkeys.len() {

            while !monkeys[monkey_idx].items.is_empty() {
                activity[monkey_idx] += 1;
                let item = monkeys[monkey_idx].items.pop_front().unwrap();
                let operand = match monkeys[monkey_idx].operand {
                    Some(operand) => {
                        operand
                    },
                    None => {
                        item
                    }
                };
                let updated_item = match monkeys[monkey_idx].operation {
                    Operation::Multiply => {
                        item * operand
                    },
                    Operation::Add => {
                        item + operand
                    }
                };
                let after_worry = if div_3 {
                    updated_item / 3
                } else {
                    updated_item
                };
                let other_monkey_idx =
                    if after_worry % monkeys[monkey_idx].test == 0 {
                        monkeys[monkey_idx].when_true
                    } else {
                        monkeys[monkey_idx].when_false
                    };
                monkeys[other_monkey_idx].items.push_back(after_worry % divisible);
            }
        }
    }

    activity.sort();
    return activity[activity.len() - 1] * activity[activity.len() - 2];

}

pub fn part1(file_path: &str) -> usize {
    calc_monkey_business(file_path, true, 20)
}

pub fn part2(file_path: &str) -> usize {
    calc_monkey_business(file_path, false, 10000)
}
