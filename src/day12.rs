use std::collections::HashSet;
use array2d::Array2D;
use crate::common::{ get_trimmed_lines, a_star };

fn make_board(lines: &Vec<String>) -> Array2D<char> {
    let rows: Vec<Vec<char>> = lines.iter().map(|line| {
        line.chars().into_iter().collect()
    }).collect();
    Array2D::from_rows(
        &rows
    ).unwrap()
}

fn find_coord(board: &Array2D<char>, item: char) -> (usize, usize) {
    for coord in board.indices_row_major() {
        if board[coord] == item {
            return coord
        }
    }

    panic!("Unexpected");
}

fn _to_score(current: char) -> u8 {
    match current {
        'S' => 1,
        'a'..='z' => {
            (current as u8) - ('a' as u8) + 1
        },
        'E' => 26,
        _ => panic!("unexpected character")
    }
}

fn find_solution(current: (usize, usize), end: (usize, usize), board: &Array2D<char>, cost: usize, prev_path: &HashSet<(usize, usize)>) -> Option<usize> {
    //println!("current {:?} {:?}", current, prev_path);
    if current == end {
        return Some(cost);
    }

    let mut results = Vec::new();
    for (neighbor_col, neighbor_row) in [
        (current.0 as i64 - 1, current.1 as i64),
        (current.0 as i64 + 1, current.1 as i64),
        (current.0 as i64, current.1 as i64 - 1),
        (current.0 as i64, current.1 as i64 + 1),
    ] {
        if neighbor_row < 0 || neighbor_col < 0 || neighbor_row >= board.row_len() as i64 || neighbor_col >= board.column_len() as i64 {
            continue;
        }

        let neighbor_coord = (neighbor_col as usize, neighbor_row as usize);
        if prev_path.contains(&neighbor_coord) {
            continue;
        }

        let mut new_path: HashSet<(usize, usize)> = prev_path.iter().map(|p| *p).collect();
        new_path.insert(current);

        let current_char = board[current];
        let neighbor_char = board[neighbor_coord];

        //println!("  neighbor {:?} {} {} {}", neighbor_coord, neighbor_char, _to_score(current_char), _to_score(neighbor_char));
        if _to_score(current_char) + 1 >= _to_score(neighbor_char) {
            let result = find_solution(neighbor_coord, end, &board, cost + 1, &new_path);
            match result {
                Some(_result) => {
                    results.push(_result);
                },
                None => {}
            }
        }
    }

    return results.iter().min().map(|s| *s);
}

pub fn part1(file_path: &str) -> usize {
    let lines = get_trimmed_lines(file_path);

    let board: Array2D<char> = make_board(&lines);
    let board_scores: Array2D<u8> = Array2D::from_iter_row_major(
        board.elements_row_major_iter().map(|c| _to_score(*c)),
        board.num_rows(),
        board.num_columns()
    ).unwrap();

    let start = find_coord(&board, 'S');
    let end = find_coord(&board, 'E');

    let path = a_star(&board_scores, start, end).unwrap();
    path.len() - 1
}

pub fn part2(file_path: &str) -> usize {
    let lines = get_trimmed_lines(file_path);
    let board: Array2D<char> = make_board(&lines);
    let board_scores: Array2D<u8> = Array2D::from_iter_row_major(
        board.elements_row_major_iter().map(|c| _to_score(*c)),
        board.num_rows(),
        board.num_columns()
    ).unwrap();

    let end = find_coord(&board, 'E');
    board_scores.enumerate_row_major().filter(|(coord, value)| {
        **value == 1
    }).filter_map(|(coord, value)| {
        a_star(&board_scores, coord, end)
    }).map(|path| path.len()).min().unwrap() - 1
}