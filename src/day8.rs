use itertools::{any, iproduct};
use crate::common::parse_digit_grid;



pub fn part1(file_path: &str) -> usize {
    let array = parse_digit_grid(file_path);

    let mut visible_count = 0;

    for (row, col) in array.indices_row_major() {
        let n = array[(row, col)];

        let mut is_visible = false;

        let iterators: Vec<Vec<(usize, usize)>> = vec![
            (0..row).map(|_r| (_r, col)).collect(),
            (row + 1..array.row_len()).map(|_r| (_r, col)).collect(),
            (0..col).map(|_c| (row, _c)).collect(),
            (col + 1..array.column_len()).map(|_c| (row, _c)).collect(),
        ];

        let results = iterators.iter().map(|iterator| {
            let mut is_visible = true;
            for (_row, _col) in iterator {
                if array[(*_row, *_col)] >= n && !(*_row == row && *_col == col) {
                    is_visible = false;
                    break;
                }
            }

            is_visible
        });


        if any(results, |r| r) {
            visible_count += 1;
        }
    }

    visible_count
}

pub fn part2(file_path: &str) -> usize {
    let array = parse_digit_grid(file_path);

    let mut visible_count = 0;

    let mut best_score = 0;

    for (row, col) in array.indices_row_major() {
        let d = array[(row, col)];

        let iterators: Vec<Vec<(usize, usize)>> = vec![
            (0..row).map(|_r| (_r, col)).rev().collect(),
            (row + 1..array.row_len()).map(|_r| (_r, col)).collect(),
            (0..col).map(|_c| (row, _c)).rev().collect(),
            (col + 1..array.column_len()).map(|_c| (row, _c)).collect(),
        ];

        let mut total_score = 1;
        for iterator in iterators.iter() {
            let mut score = 0;
            for (_row, _col) in iterator {
                score += 1;

                if array[(*_row, *_col)] >= d {
                    break;
                }
            }
            total_score *= score;
        }

        if total_score > best_score {
            best_score = total_score;
        }
    }

    best_score
}

