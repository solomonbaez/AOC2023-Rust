use std::fs::File;
use std::io::{BufRead, BufReader};

// 'Game', 'game_id<usize>',':','count_0_0','color_0_0'...'count_0_n', 'color_0_n', ';'...'count_j_j', 'color_j_j',
//
// question: determine which games would have been possible
// if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes.
// What is the sum of the IDs of those  games?

enum Bag {
    R = 12,
    G = 13,
    B = 14,
}

fn main() {
    let file = File::open("src/data.txt").unwrap();
    let data: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let games: Vec<Vec<&str>> = data
        .iter()
        .map(|line| {
            let game = line.split(";").collect();
            game
        })
        .collect();

    // println!("{:?}", games);

    let mut count = 0;
    games.iter().for_each(|game| {
        if let Some(id) = is_valid(game.to_vec()) {
            count += id;
        }
    });

    println!("{}", count);
}

fn is_valid(mut game: Vec<&str>) -> Option<i32> {
    let split: Vec<&str> = game[0].split(":").collect();
    game[0] = split[1];

    let mut id_vec: Vec<u32> = Vec::new();
    for c in split[0].chars().rev() {
        if c.is_numeric() {
            id_vec.push(c.to_digit(10).unwrap());
        }
    }

    let mut value = 0 as i32;
    for line in game.iter() {
        for c in line.chars() {
            if c.is_numeric() {
                let digit = c.to_digit(10).unwrap() as i32;
                if value != 0 {
                    value = value * 10 + digit;
                } else {
                    value += digit;
                }
                println!("{c}, {value}");
            } else if c == 'r' {
                if value > Bag::R as i32 {
                    println!("Wrong!");
                    return None;
                } else {
                    value = 0;
                }
            } else if c == 'g' {
                if value > Bag::G as i32 {
                    println!("Wrong!");
                    return None;
                } else {
                    value = 0;
                }
            } else if c == 'b' {
                if value > Bag::B as i32 {
                    println!("Wrong!");
                    return None;
                } else {
                    value = 0;
                }
            }
        }
    }

    // println!("{:?}", game);
    Some({
        if id_vec.len() == 3 {
            100
        } else if id_vec.len() == 2 {
            (id_vec[1] * 10 + id_vec[0]) as i32
        } else {
            id_vec[0] as i32
        }
    })
}
