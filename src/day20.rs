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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjusted_index() {
        assert_eq!(adjusted_index(3, 5), 2);
        assert_eq!(adjusted_index(5, -7), 3);
        assert_eq!(adjusted_index(3, 2), 2);
        assert_eq!(adjusted_index(5, -3), 2);
    }

    #[test]
    fn test_shift1() {
        let numbers: Vec<i64> = vec![2, 1, -3, 3, -2, 0, 4];
        let mut indexes = Indexes { indexes: numbers.iter().map(|x| 0i64).collect() };
        shift(&mut indexes, 0, 2);
        assert_eq!(indexes.indexes, [1, 1, -2, 0, 0, 0, 0]);
        assert_eq!(apply(&numbers, &indexes), [1, -3, 2, 3, -2, 0, 4]);
    }

    #[test]
    fn test_shift2() {
        let numbers: Vec<i64> = vec![1, -3, 2, 3, -2, 0, 4];
        let mut indexes = Indexes { indexes: numbers.iter().map(|x| 0i64).collect() };
        shift(&mut indexes, 1, -3);
        assert_eq!(indexes.indexes, [0, 1, 1, 1, -3, 0, 0]);
        //indexes.indexes = vec![0, 1, 1, 1, -3, 0, 0];
        //println!("indexes {:?}", indexes);
        assert_eq!(apply(&numbers, &indexes), [1, 2, 3, -2, -3, 0, 4]);
    }

    #[test]
    fn test_shift3() {
        let numbers: Vec<i64> = vec![1, 2, -3, 0, 3, 4, -2];
        let mut indexes = Indexes { indexes: numbers.iter().map(|x| 0i64).collect() };
        shift(&mut indexes, 5, 4);
        //
        assert_eq!(indexes.indexes, [0, 0, 0, 2, -1, -1, 0]);
        // indexes.indexes = vec![0, 0, 0, 2, -1, -1, 0];
        assert_eq!(apply(&numbers, &indexes), [1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_shift4() {
        let numbers: Vec<i64> = vec![2, 1, -3, 3, -2, 0, 4];
        let mut indexes = Indexes { indexes: numbers.iter().map(|x| 0i64).collect() };
        shift(&mut indexes, 0, 2);
        //
        //assert_eq!(indexes.indexes, [0, 0, 0, 2, -1, -1, 0]);
        // indexes.indexes = vec![0, 0, 0, 2, -1, -1, 0];
        assert_eq!(apply(&numbers, &indexes), [1, -3, 2, 3, -2, 0, 4]);
    }

    #[test]
    fn test_shift5() {
        let numbers: Vec<i64> = vec![2, 1, 3, -2, -3, 0, 4];
        let mut indexes = Indexes { indexes: numbers.iter().map(|x| 0i64).collect() };
        shift(&mut indexes, 2, 2);

        assert_eq!(apply(&numbers, &indexes), [2, 1, -2, -3, 3, 0, 4]);
    }

    #[test]
    fn test_shift6() {
        let mut indexes = Indexes {
            indexes: vec![1, -1, 0, 0, 0, 0, 0]
        };
        shift(&mut indexes, 1, 2);
        //println!("{:?}", indexes);  // should be 0, 1, -1, 0, 0, 0, 0
        let inputs: Vec<i64> = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(apply(&inputs, &indexes), [1, -3, 2, 3, -2, 0, 4]);
    }
}

fn nth(numbers: &Vec<i64>, index: i64) -> i64 {
    numbers[adjusted_index(numbers.len(), index)]
}

fn apply(numbers: &Vec<i64>, indexes: &Indexes) -> Vec<i64> {
    numbers.iter().enumerate().map(|(idx, num)| {
        nth(&numbers, idx as i64 + indexes.indexes[idx])
    }).collect()
}

fn mix (numbers: &Vec<i64>) -> Vec<i64> {
    let mut _indexes: Vec<usize> = (0..numbers.len()).map(|x| x).collect();
    let mut _numbers=  numbers.clone();
    //println!("initial {:?}", numbers);

    for (old_index, shift) in numbers.iter().enumerate() {
        let _old_index = _indexes.iter().enumerate().find_map(|(idx, x)| {
            if *x == old_index {
                Some(idx)
            } else {
                None
            }
        }).unwrap();
        let adjusted = _old_index;

        let mut new_index = adjusted as i64 + *shift;
        if new_index < 0 {
            new_index = (numbers.len() as i64 - (-new_index % numbers.len() as i64)) - 1;
            //new_index = new_index + numbers.len() as i64 - 1;
        }
        new_index = new_index % numbers.len() as i64;

        if new_index > adjusted as i64 + 1 {
            //new_index -= 1;
        }

        //println!("old: {:?}", _numbers);
        let _shift = _numbers.remove(adjusted);
        let _old_index = _indexes.remove(adjusted);
        _numbers.insert(new_index as usize, _shift);
        _indexes.insert(new_index as usize, _old_index);
        //println!("{:?} {:?}, old_index {}, new_index {}", _numbers, _indexes, adjusted, new_index);
    }

    //apply(&_numbers, &indexes)
    _numbers
}

pub fn part1(file_path: &str) -> i64 {
    let numbers: Vec<i64> = get_trimmed_lines(file_path).iter().map(|line| line.parse().unwrap()).collect();

    let mixed = mix(&numbers);
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
    0
}
