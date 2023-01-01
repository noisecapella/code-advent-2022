use std::collections::HashSet;
use std::rc::Rc;
use array2d::Array2D;
use crate::common::get_trimmed_lines;

fn parse_board(file_path: &str) -> Board {
    let rows: Vec<Vec<char>> = get_trimmed_lines(file_path).iter().map(|line| {
        let chars: Vec<char> = line.chars().collect();
        chars
    }).collect();

    let board = Array2D::from_rows(&rows).unwrap();

    let blizzards = board.indices_row_major().filter_map(|(row, col)| {
        let c = board[(row, col)];
        match c {
            'v' => Some(Blizzard { direction: (1, 0), loc: (row, col) }),
            '^' => Some(Blizzard { direction: (-1, 0), loc: (row, col) }),
            '<' => Some(Blizzard { direction: (0, -1), loc: (row, col) }),
            '>' => Some(Blizzard { direction: (0, 1), loc: (row, col) }),
            _ => None
        }
    }).collect();
    Board {
        board,
        blizzards
    }
}

fn print_board(board: &Board) {
    for row in 0..board.board.num_rows() {
        for col in 0..board.board.num_columns() {
            print!("{}", board.board[(row, col)]);
        }
        println!();
    }
    println!();
    println!();
}

type Coord = (usize, usize);

struct Blizzard {
    loc: Coord,
    direction: (i64, i64)
}

struct Board {
    board: Array2D<char>,
    blizzards: Vec<Blizzard>
}

fn move_board(board: &Board) -> Board {
    let mut new_board = Array2D::filled_with('.', board.board.num_rows(), board.board.num_columns());

    for row in 0..board.board.num_rows() {
        for col in 0..board.board.num_columns() {
            let c = if row < 1 || row >= board.board.num_rows() - 1 {
                board.board[(row, col)]
            } else if col < 1 || col >= board.board.num_columns() - 1 {
                board.board[(row, col)]
            } else {
                '.'
            };

            new_board[(row, col)] = c;
        }
    }

    let new_blizzards: Vec<_> = board.blizzards.iter().map(|blizzard| {
        let loc = (
            (((blizzard.loc.0 as i64 - 1) + blizzard.direction.0).rem_euclid(board.board.num_rows() as i64 - 2) + 1) as usize,
            (((blizzard.loc.1 as i64 - 1) + blizzard.direction.1).rem_euclid(board.board.num_columns() as i64 - 2) + 1) as usize
        );
        Blizzard {
            loc,
            direction: blizzard.direction
        }
    }).collect();

    for new_blizzard in new_blizzards.iter() {
        let c = match new_blizzard.direction {
            (-1, 0) => '^',
            (1, 0) => 'v',
            (0, -1) => '<',
            (0, 1) => '>',
            _ => panic!("unexpected")
        };

        let old_val = new_board[new_blizzard.loc];
        new_board[new_blizzard.loc] = match old_val {
            '.' => c,
            'v'|'^'|'<'|'>' => '2',
            '2'|'3'|'4'|'5'|'6'|'7'|'8' => {
                let old_digit = (old_val.to_digit(10).unwrap() + 1);
                old_digit.to_string().chars().nth(0).unwrap()
            },
            _ => panic!("unexpected")

        }
    }

    Board {
        board: new_board,
        blizzards: new_blizzards
    }
}

fn find_start_and_end(board: &Board) -> (Coord, Coord) {
    let start = (0..board.board.num_columns()).find_map(|col| {
        if board.board[(0, col)] != '#' {
            Some((0, col))
        } else {
            None
        }
    }).unwrap();

    let end = (0..board.board.num_columns()).find_map(|col| {
        if board.board[(board.board.num_rows() - 1, col)] != '#' {
            Some((board.board.num_rows() - 1, col))
        } else {
            None
        }
    }).unwrap();

    (start, end)
}

fn find_minimum_path(board: Rc<Board>, start: Coord, end: Coord) -> (Rc<Board>, usize) {
    let mut current_board: Rc<Board> = board;
    let mut old_moves: HashSet<Coord> = [start].iter().map(|c| *c).collect();
    let mut minutes = 0;
    loop {
        let next_board = Rc::new(move_board(&current_board));

        let new_moves: HashSet<_> = old_moves.iter().map(|old_move| {
            let possibilities: Vec<Coord> = [
                (0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)
            ].iter().filter_map(|possibility| {
                let new_coord = ((possibility.0 as i64 + old_move.0 as i64), (possibility.1 as i64 + old_move.1 as i64));

                if new_coord.0 < 0 || new_coord.0 >= next_board.board.num_rows() as i64 || new_coord.1 < 0 || new_coord.1 >= next_board.board.num_columns() as i64 {
                    return None;
                }

                if next_board.board[(new_coord.0 as usize, new_coord.1 as usize)] != '.' {
                    return None;
                }

                Some((new_coord.0 as usize, new_coord.1 as usize))
            }).collect();
            possibilities
        }).flatten().collect();

        for _move in new_moves.iter() {
            if *_move == end {
                return (next_board, minutes + 1);
            }
        }

        println!("minute {} {}", minutes, new_moves.len());
        print_board(&next_board);
        minutes += 1;
        old_moves = new_moves;
        current_board = next_board.clone();
    }
}

pub fn part1(file_path: &str) -> i64 {
    let board = parse_board(file_path);
    let (start, end) = find_start_and_end(&board);
    print_board(&board);

    let (new_board, minutes) = find_minimum_path(Rc::new(board), start, end);
    minutes as i64
}

pub fn part2(file_path: &str) -> i64 {
    let board = parse_board(file_path);
    let (start, end) = find_start_and_end(&board);
    print_board(&board);

    let (end_1st_board, end_1st_minutes) = find_minimum_path(Rc::new(board), start, end);
    let (start_2nd_board, start_2nd_minutes) = find_minimum_path(end_1st_board, end, start);
    let (end_2nd_board, end_2nd_minutes) = find_minimum_path(start_2nd_board, start, end);
    (end_1st_minutes + start_2nd_minutes + end_2nd_minutes) as i64
}