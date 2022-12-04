use std::{env, fs};
use std::detect::__is_feature_detected::sha;

fn calc_score(tups: Vec<Vec<&str>>) -> u64 {
    let mut score = 0;
    for tup in tups {
        let opponent_choice = tup[0];
        let my_choice = tup[1];
        let shape_score = match my_choice {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("unknown character")
        };
        let choice_score = match my_choice {
            "X" => match opponent_choice {  // i have rock
                "A" => 3,  // they have rock
                "B" => 0,  // they have paper
                "C" => 6,  // they have scissors
                _ => panic!("unknown choice")
            },
            "Y" => match opponent_choice {  // i have paper
                "A" => 6,  // they have rock
                "B" => 3,  // they have paper
                "C" => 0,  // they have scissors
                _ => panic!("unknown choice")
            },
            "Z" => match opponent_choice {  // i have scissors
                "A" => 0,  // they have rock
                "B" => 6,  // they have paper
                "C" => 3,  // they have scissors
                _ => panic!("unknown choice")
            },
            _ => panic!("unknown choice")
        };
        score += shape_score + choice_score;
    }
    score
}

pub fn day2_part1() -> u64 {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();

    let tups: Vec<Vec<&str>> = contents
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| line.split(" ").collect())
        .collect();

    calc_score(tups)
}

pub fn day2_part2() -> u64 {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();

    let tups: Vec<Vec<&str>> = contents
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| line.split(" ").collect())
        .collect();

    let adjusted_pairs: Vec<Vec<&str>> = tups.iter().map(
        |pair| {
            let opponent_choice = pair[0];
            let outcome = pair[1];

            let my_choice = match opponent_choice {
                "A" => match outcome {  // opponent chooses rock
                    "X" => "Z",  // lose
                    "Y" => "X",  // draw
                    "Z" => "Y",  // win
                    _ => panic!("unknown"),
                },
                "B" => match outcome { // opponent chooses paper
                    "X" => "X",  // lose
                    "Y" => "Y",  // draw
                    "Z" => "Z",  // win
                    _ => panic!("unknown")
                },
                "C" => match outcome {  // opponent chooses scissors
                    "X" => "Y",  // lose
                    "Y" => "Z",  // draw
                    "Z" => "X",  // win
                    _ => panic!("unknown")
                },
                _ => panic!("unknown")
            };
            vec![opponent_choice, my_choice]
        }
    ).collect();

    calc_score(adjusted_pairs)
}
