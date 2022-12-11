use std::collections::HashSet;
use std::fs::create_dir;
use crate::common::get_trimmed_lines;


pub fn part1(file_path: &str) -> usize {
    let lines = get_trimmed_lines(file_path);
    let pairs = lines.iter().map(|line| {
        let pairs: Vec<&str> = line.split(" ").collect();
        pairs
    });

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut tail_history: HashSet<(i32, i32)> = HashSet::new();
    tail_history.insert((0, 0));
    for pair in pairs {
        let direction = pair[0];

        let _move = match direction {
            "U" => {
                (1, 0)
            },
            "D" => {
                (-1, 0)
            },
            "R" => {
                (0, 1)
            },
            "L" => {
                (0, -1)
            },
            _ => panic!("Unknown direction {}", direction)
        };

        let count = pair[1].parse().unwrap();

        for _ in 0..count {
            head.0 += _move.0;
            head.1 += _move.1;

            if (tail.0 - head.0).abs() >= 2 {
                // moved up or down
                tail.0 += _move.0;
                if tail.1 != head.1 {
                    // need to move diagonally
                    tail.1 = head.1;
                }
            }

            if (tail.1 - head.1).abs() >= 2 {
                // moved right or left
                tail.1 += _move.1;
                if tail.0 != head.0 {
                    // need to move diagonally
                    tail.0 = head.0;
                }
            }

            tail_history.insert(tail);
        }
    }

    println!("tails {:?}", tail_history);
    tail_history.len()
}

pub fn print_grid(knots: &Vec<(i32, i32)>) {
    let mut bounds = ((-10, -10), (10, 10));

    let mut _expand_bounds = |tup: (i32, i32)| {
        if bounds.0.0 > tup.0 {
            bounds.0.0 = tup.0;
        }
        if bounds.1.0 <= tup.0 {
            bounds.1.0 = tup.0 + 1;
        }

        if bounds.0.1 > tup.1 {
            bounds.0.1 = tup.1;
        }
        if bounds.1.1 <= tup.1 {
            bounds.1.1 = tup.1 + 1;
        }
    };

    for knot in knots {
        _expand_bounds(*knot);
    }

    for row in bounds.0.0..bounds.1.0 {
        for col in bounds.0.1..bounds.1.1 {
            let mut c =
                if (row, col) == (0, 0) {
                    'x'
                } else if row == 0 {
                    '-'
                } else if col == 0 {
                    '|'
                } else {
                    '.'
                };
            for (idx, knot) in knots.iter().enumerate() {
                if *knot == (row, col) && (c == 'x' || c == '-' || c == '|' || c == '.') {
                    if idx == 0 {
                        c = 'H';
                    } else {
                        c = idx.to_string().chars().next().unwrap();
                    }
                }
            }
            print!("{}", c);
        }
        print!("\n");
    }
    println!();
    println!();
}

pub fn part2(file_path: &str) -> usize {
    let lines = get_trimmed_lines(file_path);
    let pairs = lines.iter().map(|line| {
        let pairs: Vec<&str> = line.split(" ").collect();
        pairs
    });

    let mut knots: Vec<(i32, i32)> = Vec::new();
    for _ in 0..10 {
        knots.push((0, 0));
    }
    let mut tail_history: HashSet<(i32, i32)> = HashSet::new();
    tail_history.insert((0, 0));
    for pair in pairs {
        let direction = pair[0];

        println!("Move {:?}", pair);
        let _move = match direction {
            "U" => {
                (-1, 0)
            },
            "D" => {
                (1, 0)
            },
            "R" => {
                (0, 1)
            },
            "L" => {
                (0, -1)
            },
            _ => panic!("Unknown direction {}", direction)
        };

        let count = pair[1].parse().unwrap();

        for _ in 0..count {
            let mut prev_knot_option: Option<(i32, i32)> = None;
            println!("move {:?}", _move);

            for mut knot in &mut knots {
                let knot_move: (i32, i32) = match prev_knot_option {
                    None => _move,
                    Some(prev_knot) => {
                        let diff = (
                            if prev_knot.0 < knot.0 { -1 } else if prev_knot.0 > knot.0 { 1 } else { 0 },
                            if prev_knot.1 < knot.1 { -1 } else if prev_knot.1 > knot.1 { 1 } else { 0 }
                        );

                        if (knot.0 - prev_knot.0).abs() >= 2 && knot.1 == prev_knot.1 {
                            diff
                        } else if (knot.1 - prev_knot.1).abs() >= 2 && knot.0 == prev_knot.0 {
                            diff
                        }
                        else if (knot.0 - prev_knot.0).abs() >= 2 || (knot.1 - prev_knot.1).abs() >= 2 {
                            let mut k_x = 0;
                            let mut k_y = 0;
                            if knot.0 < prev_knot.0 {
                                k_x = 1;
                            } else if knot.0 > prev_knot.0 {
                                k_x = -1;
                            }
                            if knot.1 < prev_knot.1 {
                                k_y = 1;
                            } else if knot.1 > prev_knot.1 {
                                k_y = -1;
                            }
                            (k_x, k_y)
                        } else {
                            (0, 0)
                        }
                    }
                };

                knot.0 += knot_move.0;
                knot.1 += knot_move.1;

                prev_knot_option = Some(*knot);
            }

            //print_grid(&knots);
            tail_history.insert(prev_knot_option.unwrap());
        }

        println!("final");
        print_grid(&knots);
    }

    //println!("tails {:?}", tail_history);
    tail_history.len()
}
