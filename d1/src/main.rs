use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let data = File::open("src/data.txt").unwrap();

    // read by line to extract calibration keys
    let reader = BufReader::new(data);

    let mut calibration_value: i32 = 0;
    let mut num_lines: i32 = 0;
    for line in reader.lines() {
        num_lines += 1;
        let mut line_calibration = vec![-1; 2];
        for s in line.unwrap().chars() {
            if let Some(c) = s.to_digit(10) {
                if line_calibration[0] == -1 {
                    line_calibration[0] = c as i32;
                } else {
                    line_calibration[1] = c as i32;
                }
            }
        }

        let mut line_calibration_collect = 0;
        if line_calibration[1] != -1 {
            line_calibration_collect += line_calibration[0] * 10 + line_calibration[1];
        } else {
            line_calibration_collect += line_calibration[0];
        }

        // println!("{}", line_calibration_collect);
        calibration_value += line_calibration_collect;
    }

    println!("calibration: {}, in {} lines", calibration_value, num_lines);
}
