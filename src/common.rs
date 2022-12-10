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
