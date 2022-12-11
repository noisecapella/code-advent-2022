use crate::common::get_trimmed_lines;

fn calc_signal_changes(lines: &Vec<String>) -> Vec<(i64, i64)> {
    let mut signal_changes: Vec<(i64, i64)> = Vec::new();
    let mut clock = 0;
    let mut x = 1;
    signal_changes.push((0, 1));
    for line in lines {
        let mut pieces = line.split(" ");
        let command = pieces.next().unwrap();

        if command == "noop" {
            clock += 1;
        } else if command == "addx" {
            clock += 2;
            let num_str = pieces.next().unwrap();
            let num: i64 = num_str.parse().unwrap();
            x += num;
            signal_changes.push((clock, x));
        } else {
            panic!("Unknown command {}", command);
        }
    }
    signal_changes
}

fn lookup_signal(key: i64, signal_changes: &Vec<(i64, i64)>) -> i64 {
    let lookup = key - 1;
    let result = signal_changes.binary_search_by_key(&lookup, |&(_clock, _strength)| _clock);
    match result {
        Ok(idx) => signal_changes[idx].1,
        Err(idx) => {
            if idx == 0 {
                //println!("err 0 {:?} {:?}", key, signal_changes);
            }
            signal_changes[idx - 1].1
        }
    }
}

pub fn part1(file_path: &str) -> i64 {
    let lines = get_trimmed_lines(file_path);

    let signal_changes = calc_signal_changes(&lines);

    println!("signal_changes {:?}", signal_changes);
    let signals = [20, 60, 100, 140, 180, 220].map(|clock| (clock, lookup_signal(clock, &signal_changes)));

    println!("signals {:?}", signals);
    signals.iter().map(|&(clock, strength) | clock * strength).sum()
}

pub fn part2(file_path: &str) -> String {
    let lines = get_trimmed_lines(file_path);

    let signal_changes = calc_signal_changes(&lines);

    let mut pixels: Vec<char> = Vec::new();
    pixels.push('\n');
    for clock_y in 0..6 {
        for clock_x in 0..40 {
            let clock = 40*clock_y + clock_x;
            let signal = lookup_signal(clock + 1, &signal_changes);
            println!("signal {:?} {:?}", signal, clock_x);
            let has_pixel = signal - 1 <= clock_x && signal + 1 >= clock_x;

            if has_pixel {
                pixels.push('#');
            } else {
                pixels.push('.');
            }
        }
        pixels.push('\n');
    }

    pixels.iter().collect()
}