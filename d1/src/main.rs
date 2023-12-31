use std::fs::File;
use std::io::{BufRead, BufReader};

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let data = File::open("src/data.txt").unwrap();
    let reader = BufReader::new(data);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let calibration_value: i32 = lines.iter().map(|line| parse_line(line.to_string())).sum();

    // ANSWER==54265
    println!("calibration: {}", calibration_value);
}

fn parse_line(line: String) -> i32 {
    let line_chars: Vec<char> = line.chars().collect();
    let mut first = -1;
    let mut second = -1;

    let get_keys = |i: usize, reverse: bool| -> Option<i32> {
        match line_chars[i].is_numeric() {
            true => Some(line_chars[i].to_digit(10).unwrap() as i32),
            false => {
                for (number, digit) in DIGITS.iter().enumerate() {
                    if reverse && line[0..i + 1].contains(digit) {
                        Some((number + 1) as i32)
                    } else if {
                        continue;
                    };
                }

                None
            }
        }
    };

    for i in 0..line.len() {
        if let Some(key) = get_keys(i, false) {
            first = key;
        }

        if first != -1 {
            break;
        }
    }

    for j in (0..line.len()).rev() {
        if let Some(key) = get_keys(j, true) {
            second = key;
        }

        if second != -1 {
            break;
        }
    }

    println!("{first}{second}");

    if second == -1 {
        first
    } else {
        first * 10 + second
    }
}
