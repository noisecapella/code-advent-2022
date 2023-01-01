use crate::common::get_trimmed_lines;

fn parse_snafu(num_str: &str) -> i64 {
    let mut n = 0;
    for c in num_str.chars() {
        n *= 5;
        n += match c {
            '0' | '1' | '2' | '3' | '4' => c.to_digit(5).unwrap() as i64,
            '-' => -1,
            '=' => -2,
            _ => panic!("unknown")
        };
    }
    n
}

fn output_snafu(n: i64) -> String {
    let mut number = n;

    let mut snafu: Vec<char> = Vec::new();
    let mut carryover = 0;

    while number != 0 || carryover != 0 {
        let remainder = (number + carryover) % 5;
        let leftover = if remainder == 0 && carryover > 0 {
            1
        } else { 0 };

        let (carry, digit) = match remainder {
            3 => (1, '='),
            4 => (1, '-'),
            0 => (0, '0'),
            1 => (0, '1'),
            2 => (0, '2'),
            _ => panic!("unexpected")
        };

        number /= 5;
        snafu.push(digit);
        carryover = carry + leftover;
    }

    snafu.reverse();
    snafu.into_iter().collect()
}

#[test]
fn test_snafu() {
    for (dec, snafu) in [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ] {
        assert_eq!(dec, parse_snafu(snafu));
        assert_eq!(snafu, output_snafu(dec));
    }
}

pub fn part1(file_path: &str) -> String {
    let lines = get_trimmed_lines(file_path);

    let sum = lines.iter().map(|line| parse_snafu(line)).sum();
    output_snafu(sum)
}
