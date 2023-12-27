use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/data.txt").unwrap();
    let data: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let cards: Vec<Vec<&str>> = data.iter().map(|line| line.split("|").collect()).collect();
    println!("{:?}", cards[0]);
}
