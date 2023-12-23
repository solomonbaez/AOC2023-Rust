use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/data.txt").unwrap();
    let data: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut parts: Vec<Vec<i32>> = vec![vec![-1; data[0].len()]; data.len()];
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

//TODO!() condense this logic within the main function -> this could be captured in a condensed closure
fn find_neighbors(row: usize, col: usize, data: &Vec<Vec<char>>, parts: &mut Vec<Vec<i32>>) -> i32 {
    //helper closure
    let mut validate_neighbors = |row: usize, col: usize| {
        let mut i = col;
        while i < data.len() && data[row][i].is_numeric() && parts[row][i] == -1 {
            parts[row][col] = data[row][col].to_digit(10).unwrap() as i32;
            i += 1;
        }

        let mut j = col - 1;
        while j > 0 && data[row][j].is_numeric() && parts[row][j] == -1 {
            parts[row][j] = data[row][j].to_digit(10).unwrap() as i32;
            j -= 1;
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

    println!("{:?}", parts);
    let mut sum = 0;
    let mut delta = 1;
    for i in (0..parts[row].len()).rev() {
        sum += parts[row][i] * delta;

        if parts[row][i] == -1 || (i > 0 && parts[row][i - 1] == -1) {
            delta = 1;
            continue;
        }

        delta *= 10;
    }

    return sum;
}
