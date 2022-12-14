use crate::common::get_trimmed_lines;
use serde_json::{ Value, json };
use std::cmp::{ Ordering, min };

fn in_right_order(value1: &Value, value2: &Value) -> Ordering {
    match value1 {
        Value::Array(_value1) => {
            match value2 {
                Value::Array(_value2) => {
                    for idx in 0..min(_value1.len(), _value2.len()) {
                        match in_right_order(&_value1[idx], &_value2[idx]) {
                            Ordering::Less => {
                                return Ordering::Less
                            },
                            Ordering::Greater => {
                                return Ordering::Greater
                            },
                            Ordering::Equal => {}
                        }
                    }

                    _value1.len().cmp(&_value2.len())
                },
                Value::Number(_value2) => {
                    let number2 = _value2.as_i64().unwrap();
                    let value2 = json!(number2);
                    in_right_order(&value1, &Value::Array(vec![value2]))
                },
                _ => panic!("Unknown object")
            }
        },
        Value::Number(_value1) => {
            let number1 = _value1.as_i64().unwrap();
            match value2 {
                Value::Array(_value2) => {
                    let value1 = json!(number1);
                    in_right_order(&Value::Array(vec![value1]), &value2)
                },
                Value::Number(_value2) => {
                    let number2 = _value2.as_i64().unwrap();
                    number1.cmp(&number2)
                },
                _ => panic!("Unknown object")
            }
        }
        _ => panic!("Unknown object")
    }
}

pub fn part1(file_path: &str) -> u64 {
    let lines = get_trimmed_lines(file_path);

    let mut sum: u64 = 0;

    for idx in (0..lines.len()).step_by(2) {
        let value1 = serde_json::from_str(lines[idx].as_str()).unwrap();
        let value2 = serde_json::from_str(lines[idx + 1].as_str()).unwrap();

        match in_right_order(&value1, &value2) {
            Ordering::Less => {
                sum += (idx as u64 / 2) + 1;
            },
            Ordering::Greater => {},
            Ordering::Equal => {
                panic!("??");
            }
        }
    }

    sum
}

pub fn part2(file_path: &str) -> u64 {
    let mut lines = get_trimmed_lines(file_path);
    lines.push("[[2]]".to_string());
    lines.push("[[6]]".to_string());

    let mut values: Vec<Value> = lines.iter().map(|line| {
        serde_json::from_str(line.as_str()).unwrap()
    }).collect();

    values.sort_by(in_right_order);

    let indexes: Vec<usize> = values.iter().enumerate().filter_map(|(idx, value)| {
        let string = value.to_string();
        if string == "[[2]]" || string == "[[6]]" {
            Some(idx + 1)
        } else {
            None
        }
    }).collect();
    (indexes[0] * indexes[1]) as u64
}