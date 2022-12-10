use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

fn char_to_value(char: char) -> u32 {
    match char {
        'a'..='z' => ((char as u32) - ('a' as u32)) + 1,
        'A'..='Z' => ((char as u32) - ('A' as u32)) + 26 + 1,
        _ => panic!("unexpected")
    }
}

pub fn part1(file_path: &str) -> u32 {
    let contents = fs::read_to_string(file_path).unwrap();

    let lines = contents.split("\n").map(|line| line.trim()).filter(|line| !line.is_empty());
    return lines.map(|line| {
        let piece1 = &line[..line.len() / 2];
        let piece2 = &line[line.len() / 2..];

        let mut set = HashSet::new();
        for char in piece1.chars() {
            set.insert(char);
        }
        for char in piece2.chars() {
            if set.contains(&char) {
                return char
            }
        }

        panic!("unexpected {} {}", piece1, piece2);
    }).map(char_to_value).sum();
}

pub fn part2(file_path: &str) -> u32 {
    let contents = fs::read_to_string(file_path).unwrap();

    let lines = contents.split("\n").map(|line| line.trim()).filter(|line| !line.is_empty());

    lines.into_iter().chunks(3).into_iter().map(|chunks| {
        chunks.map(|line| {
            let mut set = HashSet::new();
            for char in line.chars() {
                set.insert(char);
            }
            set
        }).collect()
    }).map(|chunksets: Vec<HashSet<char>>| {
        let first_chunk = chunksets.first().unwrap();
        let mut set = HashSet::new();
        for item in first_chunk {
            set.insert(*item);
        }
        for chunk in chunksets.iter().skip(1) {
            set = &set & chunk;
        }
        *set.iter().next().unwrap()
    }).map(char_to_value).sum()
}