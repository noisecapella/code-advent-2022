use std::{env, fs};

fn fully_contains(inner: &Vec<u32>, outer: &Vec<u32>) -> bool {
    outer[0] <= inner[0] && outer[1] >= inner[1]
}

fn partly_contains(inner: &Vec<u32>, outer: &Vec<u32>) -> bool {
    (outer[0] <= inner[0] && inner[0] <= outer[1]) || (outer[0] <= inner[1] && inner[1] <= outer[1])
}

pub fn day4_part1() -> u64 {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();

    let lines = contents.split("\n").map(|line| line.trim()).filter(|line| !line.is_empty());

    let pairs: Vec<Vec<Vec<u32>>> = lines.map(|line| {
        line.split(",").map(|piece| {
            piece.split("-").map(|number| {
                number.parse::<u32>().unwrap()
            }).collect()
        }).collect()
    }).collect();

    let mut fully_contain = 0;
    for pair in pairs {
        if fully_contains(&pair[0], &pair[1]) || fully_contains(&pair[1], &pair[0]) {
            fully_contain += 1;
        }
    }
    fully_contain
}

pub fn day4_part2() -> u64 {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();

    let lines = contents.split("\n").map(|line| line.trim()).filter(|line| !line.is_empty());

    let pairs: Vec<Vec<Vec<u32>>> = lines.map(|line| {
        line.split(",").map(|piece| {
            piece.split("-").map(|number| {
                number.parse::<u32>().unwrap()
            }).collect()
        }).collect()
    }).collect();

    let mut partly_contain = 0;
    for pair in pairs {
        if partly_contains(&pair[0], &pair[1]) || partly_contains(&pair[1], &pair[0]) {
            partly_contain += 1;
        }
    }
    partly_contain
}