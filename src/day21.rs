use std::collections::HashMap;
use std::rc::Rc;
use lazy_static::lazy_static;
use regex::Regex;
use crate::common::get_trimmed_lines;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug)]
struct Operation {
    operand: Operand,
    a: String,
    b: String,
}

#[derive(Debug)]
enum Value {
    Operation(Operation),
    Const(i64)
}

#[derive(Debug)]
struct Monkey {
    value: Value,
    name: String,
}

lazy_static! {
    static ref RE_CONST: Regex = Regex::new(r"([a-z]+): (\d+)").unwrap();
    static ref RE_OP: Regex = Regex::new(r"([a-z]+): ([a-z]+) ([\+\-\*/]) ([a-z]+)").unwrap();
}

fn parse(file_path: &str) -> HashMap<String, Rc<Monkey>> {
    let monkeys: Vec<Rc<Monkey>> = get_trimmed_lines(file_path).iter().map(|line| {
        Rc::new(match RE_CONST.captures(line) {
            Some(cap) => {
                Monkey {
                    name: cap.get(1).unwrap().as_str().to_string(),
                    value: Value::Const(cap.get(2).unwrap().as_str().parse::<i64>().unwrap())
                }
            },
            None => {
                match RE_OP.captures(line) {
                    Some(cap) => {
                        let op = cap.get(3).unwrap().as_str();
                        let a = cap.get(2).unwrap().as_str().to_string();
                        let b = cap.get(4).unwrap().as_str().to_string();
                        Monkey {
                            name: cap.get(1).unwrap().as_str().to_string(),
                            value: Value::Operation(Operation {
                                operand: match op {
                                    "+" => Operand::Add,
                                    "-" => Operand::Sub,
                                    "*" => Operand::Mul,
                                    "/" => Operand::Div,
                                    _ => { panic!("unknown operand"); }
                                },
                                a,
                                b
                            }),
                        }
                    },
                    None => {
                        panic!("no match for line {}", line);
                    }
                }
            }
        })
    }).collect();
    monkeys.into_iter().map(|monkey| (monkey.name.to_string(), monkey)).collect()
}

fn resolve(monkeys: &HashMap<String, Rc<Monkey>>, name: &str, error_on_name: Option<&str>) -> i64 {
    _resolve(monkeys, name, &mut HashMap::new(), error_on_name)
}

fn _resolve(monkeys: &HashMap<String, Rc<Monkey>>, name: &str, lookup: &mut HashMap<String, i64>, error_on_name: Option<&str>) -> i64 {
    match error_on_name {
        None => {},
        Some(_error_on_name) => {
            if name == _error_on_name {
                panic!("error")
            }
        }
    }

    if lookup.contains_key(name) {
        return lookup[name];
    }

    let monkey = &monkeys[name];
    let value = &monkey.value;
    let result = match value {
        Value::Const(_num) => *_num,
        Value::Operation(_op) => {
            let a = &_op.a;
            let b = &_op.b;
            match _op.operand {
                Operand::Add => _resolve(&monkeys, &a, lookup, error_on_name) + _resolve(&monkeys, &b, lookup, error_on_name),
                Operand::Sub => _resolve(&monkeys, &a, lookup, error_on_name) - _resolve(&monkeys, &b, lookup, error_on_name),
                Operand::Mul => _resolve(&monkeys, &a, lookup, error_on_name) * _resolve(&monkeys, &b, lookup, error_on_name),
                Operand::Div => _resolve(&monkeys, &a, lookup, error_on_name) / _resolve(&monkeys, &b, lookup, error_on_name),
            }
        }
    };
    lookup.insert(name.to_string(), result);
    result
}

pub fn part1(file_path: &str) -> i64 {
    let monkeys = parse(file_path);
    //println!("{:?}", monkeys);

    resolve(&monkeys, "root", None)
}


// 3 - (4 + x) == 0
// result = 0, return calc_solve(3, (4+x))
// result = 3, return calc_solve(3 - 4, x)
// result = -1, return -1
fn calc_solve(operand: Option<Operand>, stack: &Vec<String>, monkeys: &HashMap<String, Rc<Monkey>>, result: i64) -> i64 {
    let mut stack_popped = stack.clone();
    match stack_popped.pop() {
        None => {
            panic!("unexpected")
        },
        Some(_parent) => {
            let parent_monkey = monkeys[&_parent].clone();
            match &parent_monkey.value {
                Value::Const(_) => {
                    //humn
                    result
                },
                Value::Operation(_parent_op) => {
                    // a op b = result
                    let _operand = operand.unwrap();

                    match stack_popped.last() {
                        None => {
                            panic!("unexpected");
                        },
                        Some(_last) => {
                            let last_monkey = monkeys[_last.as_str()].clone();
                            let new_result = if _last.as_str() == _parent_op.a.as_str() {
                                // result = humn op const-side
                                let const_side = resolve(&monkeys, _parent_op.b.as_str(), Some("humn"));
                                //let humn_side = calc_solve(_last_op.operand, , stack, monkeys);

                                match _operand {
                                    Operand::Add => result - const_side,
                                    Operand::Sub => result + const_side,
                                    Operand::Mul => result / const_side,
                                    Operand::Div => result * const_side
                                }
                            } else {
                                // result = const-side op humn-side
                                let const_side = resolve(&monkeys, _parent_op.a.as_str(), Some("humn"));
                                match _operand {
                                    Operand::Add => result - const_side,
                                    Operand::Sub => const_side - result,
                                    Operand::Mul => result / const_side,
                                    Operand::Div => const_side / result
                                }
                            };
                            match &last_monkey.value {
                                Value::Const(_) => {
                                    calc_solve(None, &stack_popped, monkeys, new_result)
                                },
                                Value::Operation(_operation) => {
                                    calc_solve(Some(_operation.operand), &stack_popped, monkeys, new_result)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

}

fn solve(lookup: &HashMap<String, Vec<Rc<Monkey>>>, monkeys: &HashMap<String, Rc<Monkey>>, stack: &Vec<String>) -> i64 {
    let current = stack.last().unwrap();
    if lookup[current].len() > 1 {
        panic!("help");
    }

    let parent = lookup[current][0].clone();
    let mut stack_copy = stack.clone();
    stack_copy.push(parent.name.to_string());
    if parent.name == "root" {
        // result = humn-side - const-side
        calc_solve(Some(Operand::Sub),  &stack_copy, monkeys, 0)
    } else {
        solve(lookup, &monkeys, &stack_copy)
    }
}

pub fn part2(file_path: &str) -> i64 {
    let monkeys = parse(file_path);

    let mut lookup: HashMap<String, Vec<Rc<Monkey>>> = HashMap::new();
    for (name, monkey) in monkeys.iter() {
        match &monkey.value {
            Value::Const(_) => {},
            Value::Operation(_op) => {
                let a = &_op.a;
                let b = &_op.b;
                lookup.entry(a.to_string()).or_insert(vec![]).push(monkey.clone());
                lookup.entry(b.to_string()).or_insert(vec![]).push(monkey.clone());
            }
        }
    }

    solve(&lookup, &monkeys,&mut vec!["humn".to_string()])
}

