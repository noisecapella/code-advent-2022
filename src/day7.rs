use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

fn calc_directories(file_path: &str) -> HashMap<Vec<String>, usize> {
    let file_contents = fs::read_to_string(file_path).unwrap();

    let lines: Vec<&str> = file_contents.split("\n").map(|line| line.trim()).filter(|line| !line.is_empty()).collect();

    let mut sizes: HashMap<Vec<&str>, usize> = HashMap::new();
    let mut current_directory: Vec<&str> = Vec::new();
    let mut current_command: Option<&str> = None;
    for line in lines.iter() {
        if line.starts_with("$ ") {
            let pieces: Vec<&str> = line.split(" ").collect();
            current_command = Some(pieces[1]);
            match pieces[1] {
                "cd" => {
                    for (idx, piece) in pieces[2].split("/").enumerate() {
                        if piece == "" {
                            if idx == 0 {
                                current_directory = Vec::new();
                            }
                        } else {
                            if piece == ".." {
                                current_directory.pop();
                            } else {
                                current_directory.push(piece);
                            }
                        }
                    }
                },
                "ls" => {
                    // skip
                },
                _ => {
                    panic!("Unknown command {:?}", pieces[1]);
                }
            }

        }
        else {
            if current_command != Some("ls") {
                panic!("unknown current command {:?}", current_command);
            }

            let pieces: Vec<&str> = line.split(" ").collect();
            if pieces[0] == "dir" {
                // skip
            } else {
                let size = pieces[0].parse::<usize>().unwrap();

                let mut current_directory_clone = current_directory.clone();
                while !current_directory_clone.is_empty() {
                    sizes.entry(current_directory_clone.clone()).and_modify(|old| {
                        *old += size;
                    } ).or_insert(size);
                    current_directory_clone.pop();
                }
                sizes.entry(Vec::new()).and_modify(|old| *old += size).or_insert(size);
            }
        }
    }
    sizes.into_iter().map(|(key, value)| (key.iter().map(|path| path.to_string()).collect(), value)).collect()
}

pub fn part1(file_path: &str) -> usize {
    let sizes = calc_directories(file_path);
    sizes.iter().filter(|(key, value)| **value <= 100000).map(|(key, value)| value).sum()
}


pub fn part2(file_path: &str) -> usize {
    let sizes = calc_directories(file_path);
    let total_space = 70000000;
    let root_key = Vec::new();
    println!("sizes {:?}", sizes);
    let used_space = sizes.get(&root_key).unwrap();
    let free_space = total_space - used_space;
    println!("free: {:?}", free_space);
    let sorted_sizes = sizes.iter().sorted_by_key(|(key, value)| *value);
    println!("sorted: {:?}", sorted_sizes);
    for (key, value) in sorted_sizes {
        //println!("{:?} {:?}", key, value);
        if (*value + free_space) >= 30000000 {
            return *value
        }
    }

    panic!("No directory found");
}

