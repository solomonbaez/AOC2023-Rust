use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let data = File::open("src/data.txt").unwrap();
    let reader = BufReader::new(data);

    let digits: HashMap<&str, &str> = HashMap::from([
        ("0", "zero"),
        ("1", "one"),
        ("2", "two"),
        ("3", "three"),
        ("4", "four"),
        ("5", "five"),
        ("6", "six"),
        ("7", "seven"),
        ("8", "eight"),
        ("9", "nine"),
    ]);

    let mut reader_lines: Vec<String> = Vec::new();
    reader
        .lines()
        .for_each(|line| reader_lines.push(line.unwrap()));

    let mut find_and_replace: Vec<String> = Vec::new();
    let _ = &reader_lines.iter_mut().for_each(|line| {
        digits.iter().for_each(|(k, v)| {
            if line.contains(*v) {
                *line = line.replace(v, k);
            }
        });
        find_and_replace.push(line.to_string());
    });

    let mut calibration_value: i32 = 0;
    println!("{:?}, {}", find_and_replace, find_and_replace.len());
    find_and_replace.iter().for_each(|line| {
        let mut first = 0;
        let mut second = 0;
        for c in line.chars() {
            if c.is_numeric() {
                if first == 0 {
                    first = c.to_digit(10).unwrap() as i32;
                } else {
                    second = c.to_digit(10).unwrap() as i32;
                }
            }
        }

        if second != 0 {
            calibration_value += (first * 10) + second;
        } else {
            calibration_value += first;
        }
    });

    println!("calibration: {}", calibration_value);
}
