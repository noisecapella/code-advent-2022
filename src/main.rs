mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod common;

use clap::{ Arg, App };

fn main() {
    let matches = App::new("Code of Advent 2022")
        .arg(Arg::with_name("day").takes_value(true).required(true).value_parser(clap::value_parser!(u16).range(1..=25)))
        .arg(Arg::with_name("part").takes_value(true).required(true).value_parser(clap::value_parser!(u16).range(1..=2)))
        .arg(Arg::with_name("input").takes_value(true).required(true))
        .get_matches();



    let file_path: &String = matches.get_one("input").unwrap();
    let day: u16 = *matches.get_one("day").unwrap();
    let part: u16 = *matches.get_one("part").unwrap();

    println!("Day {}, part {}: ", day, part);
    let result: String =
        match day {
            1 => match part {
                1 => day1::part1(file_path).to_string(),
                2 => day1::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            2 => match part {
                1 => day2::part1(file_path).to_string(),
                2 => day2::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            3 => match part {
                1 => day3::part1(file_path).to_string(),
                2 => day3::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            4 => match part {
                1 => day4::part1(file_path).to_string(),
                2 => day4::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            5 => match part {
                1 => day5::part1(file_path).to_string(),
                2 => day5::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            6 => match part {
                1 => day6::part1(file_path).to_string(),
                2 => day6::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            7 => match part {
                1 => day7::part1(file_path).to_string(),
                2 => day7::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            8 => match part {
                1 => day8::part1(file_path).to_string(),
                2 => day8::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            _ => panic!("Unknown day {}", day)
        };

    println!("Result: {}", result);
}
