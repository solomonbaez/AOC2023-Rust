use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/data.txt").unwrap();
    let data: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let sum: i32 = data
        .iter()
        .map(|line| {
            let line_sum: i32 = line
                .chars()
                .filter_map(|c| match !c.is_alphanumeric() && c != '.' {
                    true => Some(1),
                    false => None,
                })
                .sum();

            line_sum
        })
        .sum();

    println!("{}", sum);
}
