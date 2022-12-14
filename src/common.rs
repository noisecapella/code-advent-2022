use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use array2d::Array2D;


pub fn get_trimmed_lines(file_path: &str) -> Vec<String> {
    let file_contents = fs::read_to_string(file_path).unwrap();

    file_contents.split("\n").map(|line| line.trim()).filter(|line| !line.is_empty()).map(|line| line.to_string()).collect()
}

pub fn parse_digit_grid(file_path: &str) -> Array2D<u8> {
    let rows: Vec<Vec<u8>> = get_trimmed_lines(file_path).iter().map(|line| {
        let row: Vec<u8> = line.chars().map(|c| {
            let digit: u8 = c.to_digit(10).unwrap() as u8;
            digit
        }).collect();

        row
    }).collect();
    let ret = Array2D::from_rows(
        &rows
    ).unwrap();
    ret
}

type CoordType = (usize, usize);
type ValueType = u8;
type CostType = f64;


fn reconstruct_path(came_from: &HashMap<CoordType, CoordType>, current: CoordType) -> VecDeque<CoordType> {
    let mut total_path: VecDeque<CoordType> = VecDeque::new();
    total_path.push_back(current);
    let mut current_mut = current;
    while came_from.contains_key(&current_mut) {
        current_mut = came_from[&current_mut];
        total_path.push_front(current_mut);
    }
    return total_path;
}

pub fn a_star(board: &Array2D<ValueType>, start: CoordType, goal: CoordType) -> Option<VecDeque<CoordType>> {
    let h = |coord: CoordType| {
        let pair0 = (coord.0 as f64 - goal.0 as f64);
        let pair1 = (coord.1 as f64 - goal.1 as f64);
        (pair0*pair0 + pair1*pair1).sqrt()
    };

    let mut open_set = HashSet::new();
    open_set.insert(start);
    let mut came_from : HashMap<CoordType, CoordType> = HashMap::new();

    let mut g_score: HashMap<CoordType, CostType> = HashMap::new();
    g_score.insert(start, 0f64);

    let mut f_score: HashMap<CoordType, CostType> = HashMap::new();
    f_score.insert(start, h(start) as f64);

    while !open_set.is_empty() {
        let current = {
            let mut lowest_coord:  Option<CoordType> = None;
            let mut lowest_cost: Option<CostType> = None;

            for coord in open_set.iter() {
                let cost = f_score.get(coord).unwrap_or(&f64::INFINITY);
                match lowest_cost {
                    None => {
                        lowest_coord = Some(*coord);
                        lowest_cost = Some(*cost);
                    },
                    Some(item) => {
                        if *cost < item {
                            lowest_coord = Some(*coord);
                            lowest_cost = Some(*cost);
                        }
                    }
                }
            }
            //println!("lowest is {:?} {:?}", lowest_coord, lowest_cost);
            lowest_coord.unwrap()
        };
        //println!("current {:?} {:?}", current, goal);

        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }

        open_set.remove(&current);
        for (neighbor_row, neighbor_col) in [
            (current.0 as i64 - 1, current.1 as i64),
            (current.0 as i64 + 1, current.1 as i64),
            (current.0 as i64, current.1 as i64 - 1),
            (current.0 as i64, current.1 as i64 + 1)
        ] {
            if neighbor_row < 0 || neighbor_col < 0 || neighbor_row as usize >= board.column_len() || neighbor_col as usize >= board.row_len() {
                continue;
            }

            let neighbor = (neighbor_row as usize, neighbor_col as usize);
            //println!("current {:?} neighbor {:?} {} {}", current, neighbor, board.row_len(), board.column_len());
            if board[current] + 1 < board[neighbor] {
                continue;
            }

            let tentative_g_score = g_score.get(&current).unwrap_or(&f64::INFINITY) + 1f64;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(neighbor));
                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                }
            }
        }
    }

    None
}