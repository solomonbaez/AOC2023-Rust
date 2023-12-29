use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/data.txt").unwrap();
    let data: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let cards: Vec<Vec<&str>> = data.iter().map(|line| line.split("|").collect()).collect();
    let total: i32 = cards.iter().map(|card| count_matches(card)).sum();

    println!("{}", total)
}

fn count_matches(card: &Vec<&str>) -> i32 {
    let targets: Vec<&str> = clean_card((card[0].split(":").collect::<Vec<&str>>())[1]);
    let numbers: Vec<&str> = clean_card(card[1]);

    let mut sum: i32 = 0;
    numbers
        .iter()
        .for_each(|number| match targets.contains(number) {
            true => match sum == 0 {
                true => sum += 1,
                false => sum *= 2,
            },
            false => (),
        });

    sum
}

fn clean_card(card: &str) -> Vec<&str> {
    card.split(" ")
        .filter(|number| !number.is_empty())
        .collect()
}
