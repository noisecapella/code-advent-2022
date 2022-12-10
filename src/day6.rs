use std::collections::{HashSet, VecDeque};
use std::fs;


fn calc_position(contents: &str, size_of_marker: usize) -> usize {
    let mut position: Option<usize> = None;
    let mut deque = VecDeque::new();
    for (i, c) in contents.chars().enumerate() {
        deque.push_back(c);
        if i < (size_of_marker - 1) {
            continue
        }

        let set: HashSet<&char> = HashSet::from_iter(deque.iter());
        if set.len() == size_of_marker {
            position = Some(i + 1);
            break;
        }
        deque.pop_front().unwrap();
    }
    position.unwrap()
}


pub fn part1(file_path: &str) -> usize {
    let file_contents = fs::read_to_string(file_path).unwrap();
    let contents = file_contents.trim();

    calc_position(contents, 4)
}

pub fn part2(file_path: &str) -> usize {
    let file_contents = fs::read_to_string(file_path).unwrap();
    let contents = file_contents.trim();

    calc_position(contents, 14)
}
