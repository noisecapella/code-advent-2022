use std::{env, fs, io};

fn parse_calories() -> Vec<Vec<u64>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();

    let lines = contents.split("\n").map(|line| line.trim());

    let mut elfs: Vec<Vec<u64>> = Vec::new();
    let mut current_group: Vec<u64> = Vec::new();
    for line in lines {
        if line.len() == 0 {
            elfs.push(current_group);
            current_group = Vec::new();
        } else {
            current_group.push(line.parse().unwrap());
        }
    }

    elfs.push(current_group);
    elfs
}


pub fn day1_part1() -> u64 {
    let elfs = parse_calories();

    let sums: Vec<u64> = elfs.iter().map(|group| group.iter().sum()).collect();
    let max = *sums.iter().max().unwrap();

    max
}


pub fn day1_part2() -> u64 {
    let elfs = parse_calories();

    let mut sums: Vec<u64> = elfs.iter().map(|group| group.iter().sum()).collect();
    sums.sort();
    let slice = &sums[sums.len() - 3..];
    let sum = slice.iter().sum::<u64>();

    sum
}