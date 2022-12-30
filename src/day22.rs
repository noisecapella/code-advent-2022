use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;
use array2d::Array2D;

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

enum Rotation {
    None,
    Quarter,
    Half,
    ThreeQuarters,
}


struct Cube {
    top: (Coord, SideDirection, HashMap<SideDirection, Rotation>),
    bottom: (Coord, SideDirection, HashMap<SideDirection, Rotation>),
    left: (Coord, SideDirection, HashMap<SideDirection, Rotation>),
    right: (Coord, SideDirection, HashMap<SideDirection, Rotation>),
    front: (Coord, SideDirection, HashMap<SideDirection, Rotation>),
    back: (Coord, SideDirection, HashMap<SideDirection, Rotation>),
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


fn rotate(side_direction: SideDirection, direction: Direction) -> (SideDirection, Rotation) {
    match side_direction {
        SideDirection::Left => {
            match direction {
                Direction::Left => SideDirection::Top
            }
        }
    }
}

fn rotate_direction(board: &Board, rotation: Rotation, position: Position) -> Position {
    // input position at edge, output position and direction at next step along cube
    let cube_side = max(board.num_columns(), board.num_rows()) / 4;

    let inner_row = position.coord.row % cube_side as i64;
    let start_row = position.coord.row - inner_row;
    let inner_col = position.coord.col % cube_side as i64;
    let start_col = position.coord.col - inner_col;

    match side_direction {
        SideDirection::Top => {
            match position.direction {
                Direction::Left => Position {
                    coord: Coord {
                        row: start_row,
                        col: start_col,
                    },
                    direction:
                }
            }
        }
    }
}

fn make_cube(board: &Board) -> Cube {
    let cube_side = max(board.num_columns(), board.num_rows()) / 4;

    let mut coords: HashSet<Coord> = HashSet::new();
    for row in (0..board.num_rows()).step_by(cube_side) {
        for col in (0..board.num_columns()).step_by(cube_side) {
            let coord = Coord { row: row as i64, col: col as i64 };
            match board.get(coord) {
                None => {},
                Some(_) => {
                    coords.insert(coord);
                }
            }
        }
    }

    let top = calc_start_position(board);
    let mut directions: HashMap<Coord, SideDirection> = HashMap::new();;
    directions.insert(top.coord, SideDirection::Top);

    while coords.len() != directions.len() {
        for coord in coords.iter() {
            match directions.get(coord) {
                Some(dir) => {
                    {
                        let up_coord = Coord {
                            row: coord.row - cube_side as i64,
                            col: coord.col
                        };
                        match board.get(up_coord) {
                            None => {},
                            Some(_) => {
                                let new_direction = rotate(*dir, Direction::Left);
                                if !directions.contains_key(&up_coord) {
                                    directions.insert(up_coord, new_direction);
                                } else {
                                    if directions[&up_coord] != new_direction {
                                        panic!("unexpected")
                                    }
                                }
                            }
                        }
                    }

                    {
                        let down_coord = Coord {
                            row: coord.row + cube_side as i64,
                            col: coord.col,
                        };
                        match board.get(down_coord) {
                            None => {},
                            Some(_) => {
                                let new_direction = rotate(*dir, Direction::Left);
                                if !directions.contains_key(&down_coord) {
                                    directions.insert(down_coord, new_direction);
                                } else {
                                    if directions[&down_coord] != new_direction {
                                        panic!("unexpected")
                                    }
                                }
                            }
                        }
                    }

                    {
                        let left_coord = Coord {
                            row: coord.row,
                            col: coord.col - cube_side as i64
                        };
                        match board.get(left_coord) {
                            None => {},
                            Some(_) => {
                                let new_direction = rotate(*dir, Direction::Left);
                                if !directions.contains_key(&left_coord) {
                                    directions.insert(left_coord, new_direction);
                                } else {
                                    if directions[&left_coord] != new_direction {
                                        panic!("unexpected")
                                    }
                                }
                            }
                        }
                    }
                    {
                        let right_coord = Coord {
                            row: coord.row,
                            col: coord.col + cube_side as i64,
                        };
                        match board.get(right_coord) {
                            None => {},
                            Some(_) => {
                                let new_direction = rotate(*dir, Direction::Left);
                                if !directions.contains_key(&right_coord) {
                                    directions.insert(right_coord, new_direction);
                                } else {
                                    if directions[&right_coord] != new_direction {
                                        panic!("unexpected")
                                    }
                                }
                            }
                        }
                    }

                },
                None => {
                    // pass
                }
            }
        }
    }

    let directions_reverse: HashMap<SideDirection, Coord> = directions.iter().map(|(k, v)| (*v, *k)).collect();

    Cube {
        top: (directions_reverse[&SideDirection::Top], SideDirection::Top),
        bottom: (directions_reverse[&SideDirection::Bottom], SideDirection::Bottom),
        left: (directions_reverse[&SideDirection::Left], SideDirection::Left),
        right: (directions_reverse[&SideDirection::Right], SideDirection::Right),
        front: (directions_reverse[&SideDirection::Front], SideDirection::Front),
        back: (directions_reverse[&SideDirection::Back], SideDirection::Back),
    }
}

fn calc_next_col(board: &Board, current: Position, diff: i64, as_cube: bool) -> Position {
    let mut new_coord = Coord {
        col: current.coord.col as i64 + diff,
        row: current.coord.row,
    };

    if as_cube {
        let cube_side = max(board.num_columns(), board.num_rows()) / 4;
        // if

        panic!("");
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

fn calc_next_row(board: &Board, current: Position, diff: i64, as_cube: bool) -> Position {
    let mut new_coord = Coord {
        col: current.coord.col,
        row: current.coord.row as i64 + diff,
    };

    if as_cube {
        let cube_side = max(board.num_columns(), board.num_rows()) / 4;

        panic!("");
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

fn calc_next(board: &Board, current: Position, as_cube: bool) -> Option<Position> {
    let next = match current.direction {
        Direction::Left => {
            calc_next_col(board, current, -1, as_cube)
        },
        Direction::Up => {
            calc_next_row(board, current, -1, as_cube)
        },
        Direction::Right => {
            calc_next_col(board, current, 1, as_cube)
        },
        Direction::Down => {
            calc_next_row(board, current, 1, as_cube)
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

fn follow_path(board: &Board, path: &Path, start: Position, as_cube: bool) -> Position {
    let mut current = start;
    for _move in path.iter() {
        match _move {
            Move::Forward(steps) => {
                for _ in 0..*steps {
                    match calc_next(board, current, as_cube) {
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


    let ending = follow_path(&board, &path, calc_start_position(&board), false);
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

    let ending = follow_path(&board, &path, calc_start_position(&board), true);
    let ret = (1000 * (ending.coord.row + 1)) + (4 * (ending.coord.col + 1)) + match ending.direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    ret as i64

}
