use std::fs::File;
use std::io::{BufRead, BufReader};

// we can do some basic pattern matching -> regardless of whether we implement regex matching
// in this case, we know our data follows the following structure:
// 'Game', 'game_id<usize>',':','count_0_0','color_0_0'...'count_0_n', 'color_0_n', ';',
//                              'count_1_0, color_1_0'...'count_j_j', 'color_j_j',
//
// NOTE: the games have sets elucidated by ';'
//
// The question is: Determine which games would have been possible
// if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes.
// What is the sum of the IDs of those  games?
//
fn main() {
    let file = File::open("src/data.txt").unwrap();
    let data: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    parse_game(data);
}

struct GameResults {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

const BAG: GameResults = GameResults {
    id: 0,
    red: 12,
    green: 13,
    blue: 14,
};

impl GameResults {
    fn is_valid(&self) -> bool {
        self.red <= BAG.red && self.green <= BAG.green && self.blue <= BAG.blue
    }
}

// we can try to make a deterministic data structure for the game
// -> game_id
fn parse_game(data: Vec<String>) {
    let games: Vec<&str> = data
        .iter()
        .map(|games| games.split(";").next().unwrap())
        .collect();

    println!("{:?}", games);
}
