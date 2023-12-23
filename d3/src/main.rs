use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/data.txt").unwrap();
    let data: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut visit = HashSet::new();
    let sum: i32 = data
        .iter()
        .enumerate()
        .map(|(row, line)| {
            let line_sum: i32 = line
                .iter()
                .enumerate()
                .filter_map(|(col, &c)| match !c.is_alphanumeric() && c != '.' {
                    true => {
                        return Some(find_neighbors(row, col, &data, &mut visit));
                    }
                    false => None,
                })
                .sum();

            line_sum
        })
        .sum();

    println!("{}", sum);
}

fn find_neighbors(
    row: usize,
    col: usize,
    data: &Vec<Vec<char>>,
    visit: &mut HashSet<(usize, usize)>,
) -> i32 {
    //helper closure
    let validate_neighbors = |row: usize, col: usize| {
        if data[row][col].is_numeric() && !visit.contains(&(row, col)) {
            data[row][col].to_digit(10).unwrap()
        } else {
            0
        }
    };

    let neighbors: Vec<(usize, usize)> = vec![
        (row + 1, col),
        (row - 1, col),
        (row, col + 1),
        (row, col - 1),
    ];

    neighbors
        .iter()
        .map(|case| validate_neighbors(case.0, case.1) as i32)
        .sum()
}
