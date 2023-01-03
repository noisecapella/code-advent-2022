use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Rem;
use array2d::Array2D;
use nalgebra::UnitQuaternion;

struct Board {
    board: Array2D<char>
}

#[derive(Debug)]
enum Turn {
    Right,
    Left
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Up,
    Left,
    Down
}

enum Move {
    Turn(Turn),
    Forward(usize)
}

type Path = Vec<Move>;

fn parse(file_path: &str) -> (Board, Path) {
    let file_contents = fs::read_to_string(file_path).unwrap();

    //let lines: Vec<String> = file_contents.split("\n").map(|line| line.trim()).map(|line| line.to_string()).collect();
    let mut board_lines: Vec<String> = Vec::new();
    let lines: Vec<&str> = file_contents.split("\n").collect();
    let mut is_board_lines = true;
    let mut path_line: Option<Vec<char>> = None;
    for line in lines.iter() {
        if line.is_empty() {
            is_board_lines = false;
            continue;
        }
        if is_board_lines {
            board_lines.push(line.to_string());
        } else {
            path_line = Some(line.chars().collect());
        }
    }

    let num_cols = board_lines.iter().max_by_key(|line| line.len()).unwrap().len();
    let rows: Vec<Vec<char>> = board_lines.iter().map(|line| {
        let mut chars: Vec<char> = line.chars().collect();

        for _ in chars.len()..num_cols {
            chars.push(' ');
        }

        chars
    } ).collect();
    let board = Board {
        board: Array2D::from_rows(&rows).unwrap()
    };

    let mut path: Vec<Move> = Vec::new();
    let mut work: Vec<char> = Vec::new();
    for c in path_line.unwrap() {
        match c {
            'L' => {
                let s: String = work.iter().collect();
                let steps = s.parse::<usize>().unwrap();
                path.push(Move::Forward(steps));
                path.push(Move::Turn(Turn::Left));
                work.clear();
            },
            'R' => {
                let s: String = work.iter().collect();
                let steps = s.parse::<usize>().unwrap();
                path.push(Move::Forward(steps));
                path.push(Move::Turn(Turn::Right));
                work.clear();
            },
            _ => {
                work.push(c);
            }
        }
    }

    if !work.is_empty() {
        let s: String = work.iter().collect();
        let steps = s.parse::<usize>().unwrap();
        path.push(Move::Forward(steps));
    }

    (board, path)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    row: i64,
    col: i64
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Increment {
    row: i64,
    col: i64
}

#[derive(Debug, Copy, Clone)]
struct Position {
    coord: Coord,
    direction: Direction,
}

impl Board {
    pub fn get(self: &Board, coord: Coord) -> Option<&char> {
        self.board.get(coord.row as usize, coord.col as usize)
    }

    pub fn num_columns(self: &Board) -> usize {
        self.board.num_columns()
    }

    pub fn num_rows(self: &Board) -> usize {
        self.board.num_rows()
    }
}

struct Cube {
    sides: HashMap<SideDirection, Side>,
    len_of_side: i64
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum SideDirection {
    Bottom,
    Top,
    Left,
    Right,
    Front,
    Back,
}

type Quat = UnitQuaternion<f64>;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Side {
    coord: Coord,
    direction: SideDirection,
    quaternion: Quat,
}

fn _calc_cube(board: &Board, sides: &mut HashMap<SideDirection, Side>, current_side: Side) {
    sides.insert(current_side.direction, current_side);

    panic!("??");

}
/*
fn calc_cube(board: &Board) -> Cube {
    let len_of_side = (max(board.num_columns(), board.num_rows()) / 4) as i64;

    let start = calc_start_position(board);
    let mut sides = HashMap::new();
    _calc_cube(board, &mut sides, Side {
        coord: start.coord,
        direction: SideDirection::Top,
        quaternion: UnitQuaternion::from_euler_angles(0f64, 0f64, 0f64),
    });

    Cube { sides, len_of_side }
}

fn lookup_side(cube: &Cube, coord: Coord) -> Option<Side> {
    cube.sides.values().find_map(|side| {
        if coord.row >= coord.row && coord.row < coord.row + cube.len_of_side && coord.col >= coord.col && coord.col < coord.col + cube.len_of_side {
            Some(*side)
        } else {
            None
        }
    })
}

fn transform(coord: Increment, transformation: &Quat, len_of_side: i64) -> Increment {
    let mut adjusted_new_coord = rotate(
        len_of_side,
        Increment {
            row: coord.row.rem_euclid(len_of_side),
            col: coord.col.rem_euclid(len_of_side),
        },
        transformation.rotation_count
    );

    Increment {
        row: if transformation.flip_row { len_of_side - 1 - adjusted_new_coord.row } else { adjusted_new_coord.row },
        col: if transformation.flip_col { len_of_side - 1 - adjusted_new_coord.col } else { adjusted_new_coord.col },
    }
}

fn transform_direction(direction: Direction, transformation: &Transformation) -> Direction {
    let mut result = direction;
    for _ in 0..transformation.rotation_count {
        result = match result {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    if transformation.flip_row {
        result = match result {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Left
        }
    };

    if transformation.flip_col {
        result = match result {
            Direction::Up => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Down,
            Direction::Left => Direction::Right,
        }
    };

    result
}

fn get_side(cube: &Cube, board: &Board, current: Position, increment: Increment) -> Position {
    let len_of_side = (max(board.num_columns(), board.num_rows()) / 4) as i64;

    let original_side = lookup_side(cube, current.coord).unwrap();
    let new_coord = Coord { row: current.coord.row + increment.row, col: current.coord.col + increment.col };
    match lookup_side(cube, new_coord) {
        Some(_side) => {
            Position { coord: new_coord, direction: current.direction }
        },
        None => {
            let new_side = match original_side {
                SideDirection::Top => {
                    match increment {
                        Increment { row: 0, col: -1 } => SideDirection::Left,
                        Increment { row: 0, col: 1 } => SideDirection::Right,
                        Increment { row: -1, col: 0 } => SideDirection::Back,
                        Increment { row: 1, col: 0 } => SideDirection::Front,
                        _ => panic!("unexpected")
                    }
                },
                _ => panic!("TODO")
            };
            let quaternion = cube.sides[&new_side].quaternion;
            let adjusted_new_coord = transform( increment, &quaternion, len_of_side);

            let new_direction = transform_direction(current.direction, &transformation);
            let new_side_coords = cube.sides[&new_side].0;
            Position {
                coord: Coord {
                    row: adjusted_new_coord.row + new_side_coords.row,
                    col: adjusted_new_coord.col + new_side_coords.col,
                },
                direction: new_direction
            }
        }
    }
}

fn calc_next_col(cube: &Cube, board: &Board, current: Position, diff: i64, as_cube: bool) -> Position {
    let mut new_coord = Coord {
        col: current.coord.col as i64 + diff,
        row: current.coord.row,
    };

    if as_cube {
        get_side(&cube, &board, current, Increment { row: 0, col: diff })
    } else {
        loop {
            match board.get(new_coord) {
                None => {
                    new_coord.col = new_coord.col.rem_euclid(board.num_columns() as i64);
                },
                Some(' ') => {
                    new_coord.col = (new_coord.col as i64 + diff);
                },
                Some(_) => {
                    break;
                }
            }
        }
        Position {
            direction: current.direction,
            coord: new_coord,
        }
    }
}

fn calc_next_row(cube: &Cube, board: &Board, current: Position, diff: i64, as_cube: bool) -> Position {
    let mut new_coord = Coord {
        col: current.coord.col,
        row: current.coord.row as i64 + diff,
    };

    if as_cube {
        get_side(cube, board, current, Increment { col: 0, row: diff })
    } else {
        loop {
            match board.get(new_coord) {
                None => {
                    new_coord.row = new_coord.row.rem_euclid(board.num_rows() as i64);
                },
                Some(' ') => {
                    new_coord.row = (new_coord.row as i64 + diff);
                },
                Some(_) => {
                    break;
                }
            }
        }
        Position {
            direction: current.direction,
            coord: new_coord,
        }
    }
}

fn calc_next(cube: &Cube, board: &Board, current: Position, as_cube: bool) -> Option<Position> {
    let next = match current.direction {
        Direction::Left => {
            calc_next_col(cube, board, current, -1, as_cube)
        },
        Direction::Up => {
            calc_next_row(cube, board, current, -1, as_cube)
        },
        Direction::Right => {
            calc_next_col(cube, board, current, 1, as_cube)
        },
        Direction::Down => {
            calc_next_row(cube, board, current, 1, as_cube)
        }
    };

    match board.get(next.coord) {
        Some('.') => {
            Some(next)
        },
        Some('#') => {
            None
        },
        Some(' ') => panic!("unexpected"),
        None => panic!("unexpected"),
        _ => panic!("unexpected")
    }
}

fn follow_path(cube: &Cube, board: &Board, path: &Path, start: Position, as_cube: bool) -> Position {
    let mut current = start;
    for _move in path.iter() {
        match _move {
            Move::Forward(steps) => {
                for _ in 0..*steps {
                    match calc_next(&cube, board, current, as_cube) {
                        None => {
                            break;
                        },
                        Some(new_position) => {
                            current = new_position;
                        }
                    }
                }
            },
            Move::Turn(turn) => {
                current = Position {
                    coord: current.coord,
                    direction: match turn {
                        Turn::Left => {
                            match current.direction {
                                Direction::Left => Direction::Down,
                                Direction::Down => Direction::Right,
                                Direction::Right => Direction::Up,
                                Direction::Up => Direction::Left,
                            }
                        },
                        Turn::Right => {
                            match current.direction {
                                Direction::Left => Direction::Up,
                                Direction::Up => Direction::Right,
                                Direction::Right => Direction::Down,
                                Direction::Down => Direction::Left
                            }
                        }
                    }
                }
            }
        }
    }

    current
}

fn calc_start_position(board: &Board) -> Position {
    for (row, col) in board.board.indices_row_major() {
        let coord = Coord { row: row as i64, col: col as i64 };
        match board.get(coord) {
            Some('.') => {
                return Position {
                    coord,
                    direction: Direction::Right,
                };
            },
            Some(_) => {
                // skip
            },
            None => {
                panic!("unexpected")
            }
        }
    }

    panic!("could not find start position");
}

pub fn part1(file_path: &str) -> i64 {
    let (board, path) = parse(file_path);
    let cube = calc_cube(&board);


    let ending = follow_path(&cube, &board, &path, calc_start_position(&board), false);
    let ret = (1000 * (ending.coord.row + 1)) + (4 * (ending.coord.col + 1)) + match ending.direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    ret as i64
}

pub fn part2(file_path: &str) -> i64 {
    let (board, path) = parse(file_path);
    let cube = calc_cube(&board);

    let ending = follow_path(&cube, &board, &path, calc_start_position(&board), true);
    let ret = (1000 * (ending.coord.row + 1)) + (4 * (ending.coord.col + 1)) + match ending.direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    ret as i64

}
*/
pub fn part1(file_path: &str) -> i64 {
    0
}

pub fn part2(file_path: &str) -> i64 {
    0
}
