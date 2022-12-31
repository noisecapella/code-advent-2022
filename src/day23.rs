use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use array2d::{Array2D, Error};
use itertools::Itertools;
use crate::common::get_trimmed_lines;


type CoordType = (i64, i64);

struct AdjBoard<T> {
    board: Array2D<T>,
    mins: CoordType,
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
    pub fn filled_with(elem: char, mins: CoordType, maxes: CoordType) -> AdjBoard<char> {

        AdjBoard {
            board: Array2D::filled_with(elem, (maxes.0 + 1 - mins.0) as usize, (maxes.1 + 1 - mins.1) as usize),
            mins
        }
    }

    fn get(&self, i: CoordType) -> char {
        let row = (i.0 as i64 - self.mins.0 as i64);
        let col = (i.1 as i64 - self.mins.1 as i64);
        if row >= 0 && row < self.board.num_rows() as i64 && col >= 0 && col < self.board.num_columns() as i64 {
            match self.board.get(row as usize, col as usize) {
                Some(_val) => {
                    *_val
                },
                None => {
                    '.'
                }
            }
        } else {
            '.'
        }
    }

    fn set(&mut self, i: CoordType, elem: char) {
        let row = (i.0 as i64 - self.mins.0 as i64);
        let col = (i.1 as i64 - self.mins.1 as i64);

        let ret = {
            if row >= 0 && row < self.board.num_rows() as i64 && col >= 0 && col < self.board.num_columns() as i64 {
                self.board.set(row as usize, col as usize, elem)
            } else {
                Err(Error::IndicesOutOfBounds(row as usize, col as usize))
            }
        };
        match ret {
            Ok(_) => (),
            Error => {
                let mins = (
                    min(self.mins.0, i.0),
                    min(self.mins.1, i.1),
                );

                let maxes = (
                    max(self.board.num_rows() as i64 - 1 + self.mins.0, i.0),
                    max(self.board.num_columns() as i64 - 1 + self.mins.1, i.1)
                );

                let mut new_board = AdjBoard::filled_with('.', mins, maxes);
                let indices: Vec<_> = new_board.indices_row_major().collect();
                for coord in indices {
                    let val =
                        if coord == i {
                            elem
                        } else {
                            self.get(coord)
                        };

                    if val != '.' {
                        new_board.set(coord, val);
                    }
                }
                self.board = new_board.board;
                self.mins = new_board.mins;
            }
        }
    }
}

#[test]
fn test_board() {
    let mut board = AdjBoard::filled_with('.', (0, 0), (1, 1));

    const VALS: [(i64, i64); 6] = [
        (0, 0),
        (-2, -2),
        (3, 3),
        (-4, -4),
        (4, 4),
        (-2, 4)
    ];

    for val in VALS.iter() {
        assert_eq!(board.get(*val), '.');
    }

    for (i, val) in VALS.iter().enumerate() {
        board.set(*val, i.to_string().chars().nth(0).unwrap());
    }

    for (i, val) in VALS.iter().enumerate() {
        assert_eq!(board.get(*val), i.to_string().chars().nth(0).unwrap());
    }

    assert_eq!(board.mins, (-4, -4));
    assert_eq!(board.indices_row_major().min_by_key(|c| c.0).unwrap().0, -4);
    assert_eq!(board.indices_row_major().max_by_key(|c| c.0).unwrap().0, 4);
    assert_eq!(board.indices_row_major().min_by_key(|c| c.1).unwrap().1, -4);
    assert_eq!(board.indices_row_major().max_by_key(|c| c.1).unwrap().1, 4);

    print_board(&board);
}


fn read_board(file_path: &str) -> AdjBoard<char> {
    let rows: Vec<Vec<char>> = get_trimmed_lines(file_path).iter().map(|line| {
        let cs: Vec<char> = line.chars().collect();
        cs
    }).collect();
    let array2d = Array2D::from_rows(&rows).unwrap();

    AdjBoard {
        board: array2d,
        mins: (0, 0)
    }
}

fn print_board(board: &AdjBoard<char>) {
    let row_range= -3..board.row_len() + 3;

    for row in row_range {
        let col_range = -3..board.column_len() + 3;

        for col in col_range {
            let coord = (row + board.mins.0, col + board.mins.1);
            let piece = board.get(coord);
            let c = if piece != '.' {
                piece
            } else if coord.0 == 0 && coord.1 == 0 {
                '+'
            } else if coord.0 == 0 {
                '-'
            } else if coord.1 == 0 {
                '|'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }

    println!();
    println!();
}

enum ElfDirection {
    North,
    South,
    East,
    West
}

#[derive(Debug, Copy, Clone)]
struct Elf {
    coord: CoordType,
    elfnum: usize,
}

fn calc_moves(board: &AdjBoard<char>, directions: &VecDeque<ElfDirection>) -> Vec<(Elf, CoordType)> {
    let elves: Vec<Elf> = board.indices_row_major().filter(|coord| {
        board.get(*coord) == '#'
    }).enumerate().map(|(elfnum, coord)| { Elf{ coord, elfnum }}).collect();

    let unsurrounded_elves: Vec<Elf> = elves.iter().filter_map(|elf| {
        for diff_row in [-1, 0, 1] {
            for diff_col in [-1, 0, 1] {
                let adjacent_coord = (elf.coord.0 + diff_row, elf.coord.1 + diff_col);
                if adjacent_coord == elf.coord {
                    continue;
                }

                if board.get(adjacent_coord) == '#' {
                    return Some(*elf);
                }
            }
        }
        None
    }).collect();

    let proposals: Vec<(Elf, CoordType)> = unsurrounded_elves.iter().filter_map(|elf| {
        let other_elf = directions.iter().find_map(|direction| {
            let to_check: [(i64, i64); 3] = match *direction {
                ElfDirection::North => [(-1, -1), (-1, 0), (-1, 1)],
                ElfDirection::South => [(1, -1), (1, 0), (1, 1)],
                ElfDirection::West => [(-1, -1), (0, -1), (1, -1)],
                ElfDirection::East => [(-1, 1), (0, 1), (1, 1)]
            };

            let found_other = to_check.iter().find_map(|(other_row, other_col)| {
                let other_coord = (elf.coord.0 + other_row, elf.coord.1 + other_col);
                match board.get(other_coord) {
                    '#' => Some(other_coord),
                    _ => None
                }
            });

            let on_success = match *direction {
                ElfDirection::North => (-1, 0),
                ElfDirection::South => (1, 0),
                ElfDirection::West => (0, -1),
                ElfDirection::East => (0, 1),
            };

            match found_other {
                None => Some((*elf, (elf.coord.0 + on_success.0, elf.coord.1 + on_success.1))),
                Some(_) => None
            }
        });
        other_elf
    }).collect();

    let dests: HashSet<CoordType> = proposals.iter().sorted_by_key(|(src, dest)| {
        dest
    }).group_by(|(src, dest)| {
        *dest
    }).into_iter().filter_map(|(key, group)| {
        if group.count() == 1 {
            Some(key)
        } else {
            None
        }
    }).collect();

    let valid_proposals: Vec<(Elf, CoordType)> = proposals.iter().filter_map(|(src, dest)| {
        if dests.contains(dest) {
            Some((*src, *dest))
        } else {
            None
        }
    }).collect();

    valid_proposals
}

fn simulate_round(board: &mut AdjBoard<char>, directions: &mut VecDeque<ElfDirection>) -> Vec<(Elf, CoordType)> {
    let valid_proposals = calc_moves(&board, &directions);

    for (src, dest) in valid_proposals.iter() {
        board.set(src.coord, '.');
    }

    for (src, dest) in valid_proposals.iter() {
        board.set(*dest, '#');
    }

    directions.rotate_left(1);

    valid_proposals
}

pub fn part1(file_path: &str) -> i64 {
    let mut board = read_board(file_path);

    let mut directions: VecDeque<ElfDirection> = VecDeque::from([ElfDirection::North, ElfDirection::South, ElfDirection::West, ElfDirection::East]);

    print_board(&board);
    for round in 0..10 {
        simulate_round(&mut board, &mut directions);

        println!("End of round {}", round + 1);
        print_board(&board);
    }

    let elfcoords: Vec<CoordType> = board.indices_row_major().filter(|coord| {
        board.get(*coord) == '#'
    }).collect();
    let mins = (
        elfcoords.iter().min_by_key(|coord| coord.0).unwrap().0,
        elfcoords.iter().min_by_key(|coord| coord.1).unwrap().1
    );
    let maxs = (
        elfcoords.iter().max_by_key(|coord| coord.0).unwrap().0,
        elfcoords.iter().max_by_key(|coord| coord.1).unwrap().1
    );

    let mut count_empty = 0;
    for row in mins.0..(maxs.0 + 1) {
        for col in mins.1..(maxs.1 + 1) {
            if board.get((row, col)) == '.' {
                count_empty += 1;
            }
        }
    }

    count_empty
}

pub fn part2(file_path: &str) -> i64 {
    let mut board = read_board(file_path);

    let mut directions: VecDeque<ElfDirection> = VecDeque::from([ElfDirection::North, ElfDirection::South, ElfDirection::West, ElfDirection::East]);

    print_board(&board);
    let mut round = 0;
    loop {
        let moves = simulate_round(&mut board, &mut directions);

        if moves.len() == 0 {
            break;
        }

        print_board(&board);
        round += 1;
    }

    round + 1
}
