use std::fs::File;
use std::io::{BufRead, BufReader};

// 'Game', 'game_id<usize>',':','count_0_0','color_0_0'...'count_0_n', 'color_0_n', ';'...'count_j_j', 'color_j_j',
//
// question: determine which games would have been possible
// if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes.
// What is the sum of the IDs of those  games?
//
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
        if let Some(id) = is_valid(game) {
            count += id;
        }
    });

    println!("{}", count);
}

fn is_valid(game: &Vec<&str>) -> Option<usize> {
    println!("{:?}", game);
    Some(1)
}
