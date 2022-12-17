use array2d::{Array2D, Error};
use crate::common::get_trimmed_lines;
use std::cmp::{ min, max };
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

type CoordType = (i64, i64);

struct AdjBoard<T> {
    board: Array2D<T>,
    mins: CoordType,
    floor: Option<i64>
}

impl<T> AdjBoard<T> {
    fn indices_row_major(&self) -> impl DoubleEndedIterator<Item = CoordType> + Clone + '_ {
        self.board.indices_row_major().map(|coord| {
            (coord.0 as i64 + self.mins.0 as i64, coord.1 as i64 + self.mins.1 as i64)
        })
    }

    fn row_len(&self) -> i64 {
        self.board.row_len() as i64
    }

    fn column_len(&self) -> i64 {
        self.board.column_len() as i64
    }
}

impl AdjBoard<char> {
    fn filled_with(elem: char, mins: CoordType, maxes: CoordType, floor: Option<i64>) -> AdjBoard<char> {
        let board = Array2D::filled_with(elem, (maxes.0 + 1 - mins.0) as usize, (maxes.1 + 1 - mins.1) as usize);

        AdjBoard {
            board,
            mins,
            floor
        }
    }

    fn get(&self, i: CoordType) -> Option<&char> {
        let row = (i.0 as i64 - self.mins.0 as i64);
        let col = (i.1 as i64 - self.mins.1 as i64);
        match self.floor {
            None => self.board.get(row as usize, col as usize),
            Some(_floor) => {
                if i.0 == _floor {
                    Some(&'#')
                } else {
                    match self.board.get(row as usize, col as usize) {
                        Some(_val) => {
                            Some(_val)
                        },
                        Error => {
                            Some(&'.')
                        }
                    }
                }
            }
        }
    }

    fn set(&mut self, i: CoordType, elem: char) -> Result<(), Error> {
        let row = (i.0 as i64 - self.mins.0 as i64);
        let col = (i.1 as i64 - self.mins.1 as i64);

        match self.board.get(row as usize, col as usize) {
            Some('.') => {
                // good
            },
            Some(_val) => {
                if *_val != elem && !(*_val == '+' && elem == 'o'){
                    panic!("Unexpected set");
                }
            },
            None => {
                // handle further down
            }
        };

        let ret = self.board.set(row as usize, col as usize, elem);
        match ret {
            Ok(_ret) => ret,
            Error => {
                match self.floor {
                    None => Error,
                    Some(_floor) => {
                        let mins = (
                            min(self.mins.0, i.0),
                            min(self.mins.1, i.1),
                        );

                        let maxes = (
                            max(self.board.row_len() as i64 - 1 + self.mins.0, i.0),
                            max(self.board.column_len() as i64 - 1 + self.mins.1, i.1)
                        );

                        let mut new_board = AdjBoard::filled_with('.', mins, maxes, self.floor);
                        for coord in self.indices_row_major() {
                            let val =
                                if coord == i {
                                    elem
                                } else {
                                    *self.get(coord).unwrap()
                                };

                            if val != '.' {
                                new_board.set(coord, val).unwrap();
                            }
                        }
                        self.board = new_board.board;
                        self.mins = new_board.mins;

                        Ok(())
                    }
                }
            }
        }
    }
}

fn make_board(pairs: &Vec<Vec<CoordType>>, floor: Option<i64>) -> AdjBoard<char> {
    let mut flat_pairs: Vec<&CoordType> = pairs.iter().flatten().collect();
    let source = (0, 500);
    flat_pairs.push(&source);
    let maxes = (
        flat_pairs.iter().map(|pair| pair.0).max().unwrap(),
        flat_pairs.iter().map(|pair| pair.1).max().unwrap(),
    );
    let mins = (
        flat_pairs.iter().map(|pair| pair.0).min().unwrap(),
        flat_pairs.iter().map(|pair| pair.1).min().unwrap(),
    );
    //println!("maxes mins {:?} {:?}", maxes, mins);

    let mut board = AdjBoard::filled_with('.', mins, maxes, floor);
    board.set(source, '+').unwrap();
    for list in pairs {
        let mut prev: Option<CoordType> = None;
        for pair in list {
            match prev {
                Some(_prev) => {
                    let start0 = min(_prev.0, pair.0);
                    let end0 = max(_prev.0, pair.0) + 1;
                    let start1 = min(_prev.1, pair.1);
                    let end1 = max(_prev.1, pair.1) + 1;

                    for coord0 in start0..end0 {
                        for coord1 in start1..end1 {
                            board.set((coord0, coord1), '#').unwrap();
                        }
                    }
                },
                None => {}
            }
            prev = Some(*pair);
        }
    }
    board
}

fn print_board(board: &AdjBoard<char>) {
    let row_range = match board.floor {
        Some(_floor) => -3..board.row_len() + 3,
        None => 0..board.row_len()
    };

    for row in row_range {
        let col_range = match board.floor {
            Some(_floor) => -3..board.column_len() + 3,
            None => 0..board.column_len(),
        };

        for col in col_range {
            print!("{}", board.get((row + board.mins.0, col + board.mins.1)).unwrap());
        }
        println!();
    }

    println!();
    println!();
}

fn make_pairs(file_path: &str) -> Vec<Vec<CoordType>> {
    let lines = get_trimmed_lines(&file_path);
    lines.iter().map(|line| {
        let tup_list: Vec<CoordType> = line.split(" -> ").map(|pair| {
            let mut split = pair.split(",");
            let tup0 = split.next().unwrap();
            let tup1 = split.next().unwrap();
            (tup1.parse().unwrap(), tup0.parse().unwrap())
        }).collect();
        tup_list
    }).collect()
}

#[derive(Debug)]
enum NextOpts {
    OutOfBounds,
    Blocked(CoordType),
    Viable(CoordType)
}
pub fn part1(file_path: &str) -> usize {
    let pairs = make_pairs(&file_path);
    let mut board = make_board(&pairs, None);

    let mut turns = 0;
    loop {
        let mut current: CoordType = (0, 500);

        loop {
            let next: NextOpts = {
                let mut _ret = NextOpts::Blocked(current);

                for next in [
                    (current.0 as i64 + 1, current.1 as i64),
                    (current.0 as i64 + 1, current.1 as i64 - 1),
                    (current.0 as i64 + 1, current.1 as i64 + 1),
                ] {
                    let next_value = board.get(next );
                    match next_value {
                        None => {
                            _ret = NextOpts::OutOfBounds;
                            break;
                        },
                        Some('.') => {
                            _ret = NextOpts::Viable(next);
                            break;
                        },
                        _ => {
                            // blocked
                            if current == (0, 500) {
                                return turns
                            }
                        }
                    }
                }

                _ret
            };

            match next {
                NextOpts::OutOfBounds => {
                    return turns;
                },
                NextOpts::Blocked(_current) => {
                    board.set(_current, 'o');
                    break;
                },
                NextOpts::Viable(_next) => {
                    current = _next;
                }
            }
        }

        //print_board(&board);
        turns += 1;
    }
}

pub fn part2(file_path: &str) -> usize {
    let pairs = make_pairs(&file_path);
    let floor = pairs.iter().flatten().map(|pair| pair.0).max().unwrap() + 2;
    let mut board = make_board(&pairs, Some(floor));

    let mut turns = 0;
    let mut s: HashSet<CoordType> = HashSet::new();
    loop {
        let mut current: CoordType = (0, 500);

        loop {
            match board.get(current) {
                Some('o') => {
                    return s.len();
                },
                _ => {},
            };

            let next: NextOpts = {
                let mut _ret = NextOpts::Blocked(current);

                for next in [
                    (current.0 as i64 + 1, current.1 as i64),
                    (current.0 as i64 + 1, current.1 as i64 - 1),
                    (current.0 as i64 + 1, current.1 as i64 + 1),
                ] {
                    let next_value = board.get(next );
                    match next_value {
                        None => {
                            _ret = NextOpts::OutOfBounds;
                            break;
                        },
                        Some('.') => {
                            _ret = NextOpts::Viable(next);
                            break;
                        },
                        _ => {
                            // blocked, try again
                        }
                    }
                }

                _ret
            };

            match next {
                NextOpts::OutOfBounds => {
                    panic!("infinite floor");
                },
                NextOpts::Blocked(_current) => {
                    board.set(_current, 'o').unwrap();
                    //println!("current, {:?}", current);
                    s.insert(_current);
                    break;
                },
                NextOpts::Viable(_next) => {
                    current = _next;
                }
            }
        }

        //println!("s {}", s.len() - 1);
        // print_board(&board);
        turns += 1;
    }

}