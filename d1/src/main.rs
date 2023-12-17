use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let data = File::open("src/data.txt").unwrap();
    let reader = BufReader::new(data);

    let mut calibration_value = 0;
    reader.lines().for_each(|line| {
        let digits: Vec<u32> = line
            .unwrap()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        println!("{:?}", digits);

        let line_calibration = (digits[0] * 10) + digits[digits.len() - 1];
        println!("{line_calibration}");

        calibration_value += line_calibration
    });

    println!("calibration: {}", calibration_value);
}
