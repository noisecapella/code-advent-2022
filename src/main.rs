mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
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
            9 => match part {
                1 => day9::part1(file_path).to_string(),
                2 => day9::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            10 => match part {
                1 => day10::part1(file_path).to_string(),
                2 => day10::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            11 => match part {
                1 => day11::part1(file_path).to_string(),
                2 => day11::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            12 => match part {
                1 => day12::part1(file_path).to_string(),
                2 => day12::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            13 => match part {
                1 => day13::part1(file_path).to_string(),
                2 => day13::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            14 => match part {
                1 => day14::part1(file_path).to_string(),
                2 => day14::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            15 => match part {
                1 => day15::part1(file_path).to_string(),
                2 => day15::part2(file_path).to_string(),
                _ => panic!("Unknown part {}", part)
            },
            16 => match part {
                1 => day16::part1(file_path).to_string(),
                2 => day16::part2(file_path).to_string(),
                _ => panic!("Unknown {}", part)
            },
            17 => match part {
                1 => day17::part1(file_path).to_string(),
                2 => day17::part2(file_path).to_string(),
                _ => panic!("Unknown {}", part)
            },
            18 => match part {
                1 => day18::part1(file_path).to_string(),
                2 => day18::part2(file_path).to_string(),
                _ => panic!("Unknown {}", part)
            },
            19 => match part {
                1 => day19::part1(file_path).to_string(),
                2 => day19::part2(file_path).to_string(),
                _ => panic!("Unknown {}", part)
            },
            20 => match part {
                1 => day20::part1(file_path).to_string(),
                2 => day20::part2(file_path).to_string(),
                _ => panic!("Unknown {}", part)
            },
            21 => match part {
                1 => day21::part1(file_path).to_string(),
                2 => day21::part2(file_path).to_string(),
                _ => panic!("Unknown {}", part)
            },
            22 => match part {
                1 => day22::part1(file_path).to_string(),
                2 => day22::part2(file_path).to_string(),
                _ => panic!("Unknown {}", part)
            },
            23 => match part {
                1 => day23::part1(file_path).to_string(),
                2 => day23::part2(file_path).to_string(),
                _ => panic!("Unknown {}", part)
            },
            24 => match part {
                1 => day24::part1(file_path).to_string(),
                2 => day24::part2(file_path).to_string(),
                _ => panic!("Unknown {}", part)
            },
            25 => match part {
                1 => day25::part1(file_path).to_string(),
                _ => panic!("Unknown {}", part)
            },
            _ => panic!("Unknown day {}", day)
        };

    println!("Result: {}", result);
}
