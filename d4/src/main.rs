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

    // println!("targets: {:?}, numbers: {:?}", targets, numbers);

    let sum: i32 = numbers
        .iter()
        .map(|number| {
            if targets.contains(number) {
                let mut delta = 1;
                let sub_sum: i32 = number
                    .chars()
                    .rev()
                    .map(|digit| {
                        let base_10 = (digit.to_digit(10).unwrap() as i32) * delta;
                        delta *= 10;
                        base_10
                    })
                    .sum::<i32>();
                sub_sum
            } else {
                0
            }
        })
        .sum();

    // println!("{:?}", sum);
    sum //TODO!() CURRENTLY TOO HIGH -> could be duplicate matches
}

fn clean_card(card: &str) -> Vec<&str> {
    card.split(" ")
        .filter(|number| !number.is_empty())
        .collect()
}
