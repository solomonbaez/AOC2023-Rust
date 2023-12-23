use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/data.txt").unwrap();
    let data: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut parts: Vec<Vec<i32>> = vec![vec![0; data[0].len()]; data.len()];
    let sum: i32 = data
        .iter()
        .enumerate()
        .map(|(row, line)| {
            let line_sum: i32 = line
                .iter()
                .enumerate()
                .filter_map(|(col, &c)| match !c.is_alphanumeric() && c != '.' {
                    true => {
                        return Some(find_neighbors(row, col, &data, &mut parts));
                    }
                    false => None,
                })
                .sum();

            line_sum
        })
        .sum();

    println!("{}", sum);
}

//TODO!() change sum logic to chain together full number sequences
fn find_neighbors(row: usize, col: usize, data: &Vec<Vec<char>>, parts: &mut Vec<Vec<i32>>) -> i32 {
    //helper closure
    let mut validate_neighbors = |row: usize, col: usize| {
        if data[row][col].is_numeric() && parts[row][col] == 0 {
            // println!("{:?}", visit);
            parts[row][col] = data[row][col].to_digit(10).unwrap() as i32;
            // 1
        }
    };

    //adj and diag
    let neighbors: Vec<(usize, usize)> = vec![
        (row + 1, col),
        (row - 1, col),
        (row, col + 1),
        (row, col - 1),
        (row + 1, col + 1),
        (row + 1, col - 1),
        (row - 1, col + 1),
        (row - 1, col - 1),
    ];

    neighbors
        .iter()
        .for_each(|case| validate_neighbors(case.0, case.1));

    let mut sum = 0;
    let mut delta = 1;
    for i in (0..parts[row].len()).rev() {
        sum += parts[row][i] * delta;

        if parts[row][i] == 0 || (i > 0 && parts[row][i - 1] == 0) {
            delta = 1;
            continue;
        }

        delta *= 10;
    }

    return sum;
}
