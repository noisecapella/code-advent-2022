use std::fs;

fn parse_calories(file_path: &str) -> Vec<Vec<u64>> {
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


pub fn part1(file_path: &str) -> u64 {
    let elfs = parse_calories(file_path);

    let sums: Vec<u64> = elfs.iter().map(|group| group.iter().sum()).collect();
    let max = *sums.iter().max().unwrap();

    max
}


pub fn part2(file_path: &str) -> u64 {
    let elfs = parse_calories(file_path);

    let mut sums: Vec<u64> = elfs.iter().map(|group| group.iter().sum()).collect();
    sums.sort();
    let slice = &sums[sums.len() - 3..];
    let sum = slice.iter().sum::<u64>();

    sum
}