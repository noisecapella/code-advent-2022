use std::collections::{HashMap, HashSet};
use crate::common::get_trimmed_lines;
use array2d::Array2D;
use itertools::any;
use lazy_static::lazy_static;

const NUM_COLS: usize = 7;
type Row = [char; NUM_COLS];

const NUM_ROWS_RECORD: u64 = 24;

struct Board {
    rows: Vec<Row>,
    above_last_occupied_row: u64,
    active_piece: Option<((u64, u64), u64)>,
    piece_num: u64,
}

fn empty_row() -> Row {
    ['.', '.', '.', '.', '.', '.', '.']
}

fn make_pieces() -> Vec<Vec<(u64, u64)>> {
    vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(2, 2), (1, 2), (0, 0), (0, 1), (0, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)]
    ]
}

lazy_static! {
   static ref PIECES: Vec<Vec<(u64, u64)>> = make_pieces();
}

impl Board {
    pub fn set(self: &mut Board, row: u64, col: u64, val: char) {
        if row >= self.rows.len() as u64 {
            for _ in self.rows.len()..row as usize + 1 {
                self.rows.push(empty_row())
            }
        }

        self.rows[row as usize][col as usize] = val;
        if val != '.' {
            if row >= self.above_last_occupied_row {
                self.above_last_occupied_row = row + 1;
            }
        } else {
            for _row in (0..self.rows.len()).rev() {
                if any(self.rows[_row], |x| x != '.') {
                    self.above_last_occupied_row = (_row as u64) + 1;
                    break;
                }
            }
        }
    }

    pub fn get(self: &Board, row: u64, col: u64) -> char {
        self.rows[row as usize][col as usize]
    }

    pub fn new() -> Board {
        Board {
            above_last_occupied_row: 0,
            rows: Vec::new(),
            active_piece: None,
            piece_num: 0,
        }

    }

    fn print(self: &Board) {
        for row in self.rows.iter().rev() {
            print!("|");
            for col in 0..NUM_COLS {
                let c = row[col];
                print!("{}", c);
            }
            println!("|");
        }

        print!("|");
        for _ in 0..NUM_COLS {
            print!("=");
        }
        println!("|");
    }

    fn push_active_piece(self: &mut Board, direction: i64) -> bool {
        match self.active_piece {
            None => {
                panic!("no piece to move");
            },
            Some(_active_piece) => {
                //println!();
                //println!("start");
                //self.print();

                let can_move = {
                    let mut ret = true;
                    for (row, col) in PIECES[_active_piece.1 as usize].iter() {
                        let new_col = _active_piece.0.1 as i64 + *col as i64 + direction;
                        let new_row = _active_piece.0.0 as i64 + *row as i64;
                        if new_col < 0 || new_col >= NUM_COLS as i64 || self.rows[new_row as usize][new_col as usize] == '#' {
                            ret = false;
                            break;
                        }
                    }
                    ret
                };

                if can_move {
                    for (row, col) in PIECES[_active_piece.1 as usize].iter() {
                        self.set(_active_piece.0.0 + *row, _active_piece.0.1 + *col, '.');
                    }

                    for (row, col) in PIECES[_active_piece.1 as usize].iter() {
                        self.set(_active_piece.0.0 + *row, (_active_piece.0.1 as i64 + *col as i64 + direction) as u64, '@');
                    }
                }

                //println!();
                //println!("can_move {}", can_move);
                //self.print();

                let moved_col = if can_move { direction } else { 0 };

                let can_move_down = {
                    let mut ret = true;
                    for (row, col) in PIECES[_active_piece.1 as usize].iter() {
                        let new_row = _active_piece.0.0 as i64 + *row as i64 - 1;
                        let new_col = _active_piece.0.1 as i64 + *col as i64 + moved_col;
                        if new_row < 0 || self.rows[new_row as usize][new_col as usize] == '#' {
                            ret = false;
                            break;
                        }
                    }
                    ret
                };

                let moved_row: i64 = if can_move_down { -1 } else { 0 };
                if can_move_down {
                    for (row, col) in PIECES[_active_piece.1 as usize].iter() {
                        self.set(_active_piece.0.0 + *row, (_active_piece.0.1 as i64 + *col as i64 + moved_col) as u64, '.');
                    }

                    for (row, col) in PIECES[_active_piece.1 as usize].iter() {
                        self.set((_active_piece.0.0 as i64 + *row as i64 + moved_row) as u64, (_active_piece.0.1 as i64 + *col as i64 + moved_col) as u64, '@');
                    }

                    self.active_piece = Some(((
                        (_active_piece.0.0 as i64 + moved_row) as u64,
                        (_active_piece.0.1 as i64 + moved_col) as u64
                        ), _active_piece.1));
                    false
                } else {
                    for (row, col) in PIECES[_active_piece.1 as usize].iter() {
                        self.set(_active_piece.0.0 + *row, (_active_piece.0.1 as i64 + *col as i64 + moved_col) as u64, '#');
                    }
                    self.active_piece = None;
                    true
                }

                //println!();
                //println!("can_move_down {}", can_move_down);
                //self.print();
            }
        }
    }

    fn simulate_rock(self: &mut Board, direction: char) -> bool {
        if self.active_piece.is_none() {
            let bottom_row = self.above_last_occupied_row + 3;
            let piece = &PIECES[self.piece_num as usize];

            for (row, col) in piece {
                self.set(row + bottom_row, 2 + col, '@');
            }

            self.active_piece = Some(((bottom_row, 2), self.piece_num));
            self.piece_num = (self.piece_num + 1) % (PIECES.len() as u64);

            //println!("round {}", count);
            //board.print();
        }

        match direction {
            '>' => {
                self.push_active_piece(1)
            },
            '<' => {
                self.push_active_piece(-1)
            },
            _ => {
                panic!("??")
            }
        }
    }
}

pub fn part1(file_path: &str) -> i64 {
    let lines = get_trimmed_lines(&file_path);

    let mut board = Board::new();
    let directions: Vec<char> = lines.first().unwrap().chars().collect();
    let mut direction_idx = 0;
    let mut count = 0;

    loop {
        if count == 2022 {
            break;
        }

        let direction = directions[direction_idx];
        //println!("count = {}, direction = {}, height = {}", count, direction, board.above_last_occupied_row);

        if board.simulate_rock(direction) {
            count += 1;
        }

        direction_idx = (direction_idx + 1) % directions.len();
    }

    board.above_last_occupied_row as i64
}

pub fn part2(file_path: &str) -> i64 {
    let lines = get_trimmed_lines(&file_path);

    let mut board = Board::new();

    let directions: Vec<char> = lines.first().unwrap().chars().collect();
    let mut direction_idx = 0;
    let mut count: u64 = 0;
    let mut repeat: HashMap<([[char; 7]; NUM_ROWS_RECORD as usize], u64), (u64, u64, u64)> = HashMap::new();

    loop {
        let direction = directions[direction_idx];
        //println!("count = {}, direction = {}, height = {}", count, direction, board.above_last_occupied_row);

        let new_piece = board.simulate_rock(direction);
        if new_piece {
            count += 1;
        }

        direction_idx = (direction_idx + 1) % directions.len();

        if board.above_last_occupied_row >= 24 {
            let key = ([
                board.rows[(board.above_last_occupied_row - 1) as usize],
                board.rows[(board.above_last_occupied_row - 2) as usize],
                board.rows[(board.above_last_occupied_row - 3) as usize],
                board.rows[(board.above_last_occupied_row - 4) as usize],
                board.rows[(board.above_last_occupied_row - 5) as usize],
                board.rows[(board.above_last_occupied_row - 6) as usize],
                board.rows[(board.above_last_occupied_row - 7) as usize],
                board.rows[(board.above_last_occupied_row - 8) as usize],
                board.rows[(board.above_last_occupied_row - 9) as usize],
                board.rows[(board.above_last_occupied_row - 10) as usize],
                board.rows[(board.above_last_occupied_row - 11) as usize],
                board.rows[(board.above_last_occupied_row - 12) as usize],
               board.rows[(board.above_last_occupied_row - 13) as usize],
               board.rows[(board.above_last_occupied_row - 14) as usize],
               board.rows[(board.above_last_occupied_row - 15) as usize],
               board.rows[(board.above_last_occupied_row - 16) as usize],
               board.rows[(board.above_last_occupied_row - 17) as usize],
               board.rows[(board.above_last_occupied_row - 18) as usize],
               board.rows[(board.above_last_occupied_row - 19) as usize],
               board.rows[(board.above_last_occupied_row - 20) as usize],
               board.rows[(board.above_last_occupied_row - 21) as usize],
               board.rows[(board.above_last_occupied_row - 22) as usize],
               board.rows[(board.above_last_occupied_row - 23) as usize],
               board.rows[(board.above_last_occupied_row - 24) as usize],
            ], board.piece_num);
            //println!("{:?}", repeat.len());

            if new_piece {
                //println!("repeat {}", repeat.len());
                match repeat.get(&key) {
                    Some((_count, _above_last_occupied_row, _last_piece_num)) => {
                        let row_diff = board.above_last_occupied_row - _above_last_occupied_row;
                        let rocks_diff = count - _count;
                        const TOTAL_ROCKS: u64 = 1000000000000;
                        let rounds = (TOTAL_ROCKS - _count) / rocks_diff;

                        let mut updated_rocks_count = _count + (rocks_diff * rounds);
                        while updated_rocks_count < TOTAL_ROCKS {
                            if board.simulate_rock(directions[direction_idx]) {
                                updated_rocks_count += 1;
                            }

                            direction_idx = (direction_idx + 1) % directions.len();
                        }
                        return (board.above_last_occupied_row + row_diff * (rounds - 1)) as i64;
                    },
                    None => {
                        repeat.insert(key, (count, board.above_last_occupied_row, board.piece_num));
                    }
                }
            }
        }
    }
}