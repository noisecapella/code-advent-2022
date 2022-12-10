use std::fs;
use itertools::Itertools;

pub fn puzzle(file_path: &str, part1: bool) -> String {
    let contents = fs::read_to_string(file_path).unwrap();

    let lines: Vec<&str> = contents.split("\n").map(|line| line.trim_end()).collect();

    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut linebreak: Option<usize> = None;
    for (idx, line) in lines.iter().enumerate() {
        if line.is_empty() {
            linebreak = Some(idx);
            break
        }

        for col_num in 0..(line.len()/4) + 1 {
            let c = line.chars().nth(col_num*4 + 1).unwrap();
            if line.chars().nth(col_num*4).unwrap() == '['  {
                while crates.len() <= col_num {
                    crates.push(Vec::new());
                }

                crates[col_num].push(c);
            }
        }
    }

    for col_num in 0..crates.len() {
        crates[col_num].reverse();
    }

    for line in &lines[linebreak.unwrap()+1..] {
        if line.trim().is_empty() {
            break
        }
        let pieces: Vec<&str> = line.split(' ').collect();
        let count: usize = pieces[1].parse().unwrap();
        let from: usize = pieces[3].parse().unwrap();
        let to: usize = pieces[5].parse().unwrap();

        if part1 {
            for _ in 0..count {
                let popped = crates[from - 1].pop().unwrap();

                crates[to - 1].push(popped);
            }
        } else {
            let mut intermediate = Vec::new();
            for _ in 0..count {
                let popped = crates[from - 1].pop().unwrap();
                intermediate.push(popped);
            }
            for _ in 0..count {
                let popped = intermediate.pop().unwrap();
                crates[to - 1].push(popped);
            }
        }
    }

    let response = crates.iter().map(|column| {
        column.last().unwrap()
    }).join("");

    response
}

pub fn part1(file_path: &str) -> String {
    puzzle(file_path, true)
}

pub fn part2(file_path: &str) -> String {
    puzzle(file_path, false)
}
