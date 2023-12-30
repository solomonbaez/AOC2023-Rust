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

    println!("calibration: {}", calibration_value);
}

fn parse_line(line: String) -> i32 {
    let mut first = -1;
    let mut second = -1;

    let line_chars: Vec<char> = line.chars().collect();
    for i in 0..line.len() {
        if line_chars[i].is_numeric() {
            first = line_chars[i].to_digit(10).unwrap() as i32;
            break;
        } else {
            // println!("{:?}", &line[0..i]);
            for (number, digit) in DIGITS.iter().enumerate() {
                if line[0..i + 1].contains(digit) {
                    first = (number + 1) as i32;
                    // println!("{first}");
                    break;
                }
            }
        }

        if first != -1 {
            break;
        }
    }

    for j in (0..line.len()).rev() {
        if line_chars[j].is_numeric() {
            second = line_chars[j].to_digit(10).unwrap() as i32;
            break;
        } else {
            // println!("{:?}", &line[j..line.len()]);
            for (number, digit) in DIGITS.iter().enumerate() {
                if line[j..line.len()].contains(digit) {
                    second = (number + 1) as i32;
                    // println!("{second}");
                    break;
                }
            }
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
