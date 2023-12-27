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
    count_matches(&cards[0]);
}

fn count_matches(card: &Vec<&str>) -> i32 {
    let targets: Vec<&str> = clean_card((card[0].split(":").collect::<Vec<&str>>())[1]);
    let numbers: Vec<&str> = clean_card(card[1]);

    println!("targets: {:?}, numbers: {:?}", targets, numbers);

    return 0;
}

fn clean_card(card: &str) -> Vec<&str> {
    card.split(" ")
        .filter(|number| !number.is_empty())
        .collect()
}
