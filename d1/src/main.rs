use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let data = File::open("src/data.txt").unwrap();
    let reader = BufReader::new(data);

    let mut calibration_value: i32 = 0;
    reader
        .lines()
        .for_each(|line| calibration_value += parse_line(line.unwrap()));

    println!("calibration: {}", calibration_value);
}

fn parse_line(line: String) -> i32 {
    let digits: HashMap<&str, i32> = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut l = 0;
    let mut r = 0;

    //initialize first and last counter
    let mut first = 0;
    let mut second = 0;

    //convert string to a vector representation
    let line_characters: Vec<char> = line.chars().collect();
    while r < line.len() {
        if let Some(digit) = digits.get(&line[l..r + 1]) {
            // println!("{digit}");
            if first == 0 {
                first = *digit;
            } else {
                second = *digit;
            }

            //shift left pointer
            l = r + 1;
        } else if line_characters[r].is_numeric() {
            let digit = line_characters[r].to_digit(10).unwrap();
            // println!("{digit}");

            if first == 0 {
                first = digit as i32;
            } else {
                second = digit as i32;
            }

            l = r + 1;
        }

        r += 1;
    }

    // println!("{}, {}", first, second);
    if second != 0 {
        first * 10 + second
    } else {
        first
    }
}
