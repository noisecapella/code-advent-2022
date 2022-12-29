use std::cmp::{max, min};
use crate::common::get_trimmed_lines;

#[derive(Debug)]
struct Indexes {
    indexes: Vec<i64>,
}

fn adjusted_index(numbers_len: usize, index: i64) -> usize {
    (if index >= 0 {
        (index % numbers_len as i64)
    } else {
        (numbers_len as i64 - ((index * -1) as usize % numbers_len) as i64)
    }) as usize
}

fn shift(indexes: &mut Indexes, old_index: usize, shift: i64) {
    if shift == 0 {
        return;
    }

    let indexes_len = indexes.indexes.len() - 1;
    let dest_index = adjusted_index(indexes_len, old_index as i64 + shift);
    let direction: i64 = if dest_index > old_index { -1 } else { 1 };
    let diff = dest_index as i64 - old_index as i64;

    indexes.indexes[dest_index] -= diff;
    let mut index = (dest_index as i64 + direction) as usize;
    while index != old_index {
        indexes.indexes[index] -= direction;

        index = (index as i64 + direction) as usize;
    }
    indexes.indexes[index] -= direction;
}

fn nth(numbers: &Vec<i64>, index: i64) -> i64 {
    numbers[adjusted_index(numbers.len(), index)]
}

fn mix (numbers: &Vec<i64>, count: usize) -> Vec<i64> {
    let mut _indexes: Vec<usize> = (0..numbers.len()).map(|x| x).collect();
    let mut _numbers=  numbers.clone();
    //println!("initial {:?}", numbers);

    for _ in 0..count {
        for (old_index, shift) in numbers.iter().enumerate() {
            let _old_index = _indexes.iter().enumerate().find_map(|(idx, x)| {
                if *x == old_index {
                    Some(idx)
                } else {
                    None
                }
            }).unwrap();
            let adjusted_old_index = _old_index;

            let mut new_index = adjusted_old_index as i64 + *shift;
            if new_index < 0 {
                new_index = ((numbers.len() as i64 - 1) - (-new_index % (numbers.len() as i64 - 1)));
                //new_index = new_index + numbers.len() as i64 - 1;
            }
            // 0, 1, 2, 3, 4, 5, 6
            new_index = new_index % (numbers.len() as i64 - 1);

            if new_index > adjusted_old_index as i64 {
                //new_index -= 1;
            }

            let _shift = _numbers.remove(adjusted_old_index);
            let _old_index = _indexes.remove(adjusted_old_index);
            _numbers.insert(new_index as usize, _shift);
            _indexes.insert(new_index as usize, _old_index);
            //println!("{} {:?} {:?}, old_index {}, new_index {}, orig index {}", shift, _numbers, _indexes, adjusted_old_index, new_index, old_index);
        }
    }

    //apply(&_numbers, &indexes)
    _numbers
}

pub fn part1(file_path: &str) -> i64 {
    let numbers: Vec<i64> = get_trimmed_lines(file_path).iter().map(|line| line.parse().unwrap()).collect();

    let mixed = mix(&numbers, 1);
    let zero = mixed.iter().enumerate().find_map(|(i, x)| {
        if *x == 0 {
            Some(i)
        } else {
            None
        }
    }).unwrap() as i64;
    //println!("{:?}", mixed);
    //println!("{} {} {} {}", zero, nth(&mixed, 1000 + zero), nth(&mixed, 2000 + zero), nth(&mixed, 3000 + zero));

    nth(&mixed, 1000 + zero) + nth(&mixed, 2000 + zero) + nth(&mixed, 3000 + zero)
}

pub fn part2(file_path: &str) -> i64 {
    const DECRYPTION: i64 = 811589153;
    let numbers: Vec<i64> = get_trimmed_lines(file_path).iter().map(|line| line.parse().unwrap()).map(|number: i64| number * DECRYPTION).collect();
    let mixed = mix(&numbers, 10);

    let zero = mixed.iter().enumerate().find_map(|(i, x)| {
        if *x == 0 {
            Some(i)
        } else {
            None
        }
    }).unwrap() as i64;

    println!("{} {} {} {}", zero, nth(&mixed, 1000 + zero), nth(&mixed, 2000 + zero), nth(&mixed, 3000 + zero));
    nth(&mixed, 1000 + zero) + nth(&mixed, 2000 + zero) + nth(&mixed, 3000 + zero)
}
