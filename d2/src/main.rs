use std::fs::File;
use std::io::{BufRead, BufReader};

// 'Game', 'game_id<usize>',':','count_0_0','color_0_0'...'count_0_n', 'color_0_n', ';'...'count_j_j', 'color_j_j',
//
// question: determine which games would have been possible
// if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes.
// What is the sum of the IDs of those  games?
//
static BAG: &'static [(&'static char, usize)] = &[(&'r', 12), (&'g', 13), (&'b', 14)];

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

    let mut count = 0;
    games.iter().for_each(|game| {
        if let Some(id) = is_valid(game.to_vec()) {
            count += id;
        }
    });

    println!("{}", count);
}

fn is_valid(game: Vec<&str>) -> Option<usize> {
    let mut id = 0;
    let mut value = 0;
    for line in game.iter() {
        for c in line.chars() {
            if c.is_numeric() && id == 0 {
                id = c.to_digit(10).unwrap() as usize;
            } else if c.is_numeric() && value == 0 {
                value = c.to_digit(10).unwrap() as usize;
            } else if c == 'r' {
                if value > BAG[0].1 {
                    return None;
                }

                value = 0
            } else if c == 'g' {
                if value > BAG[1].1 {
                    return None;
                }

                value = 0
            } else if c == 'b' {
                if value > BAG[2].1 {
                    return None;
                }

                value = 0;
            }
        }
    }

    // println!("{:?}", game);
    Some(id)
}
