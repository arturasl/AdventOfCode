use itertools::Itertools;
use std::io::{self, BufRead};
use std::thread;

const EXPAND: usize = 1_000_000;

fn run() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().trim().chars().collect::<Vec<char>>())
        .collect();
    let mut galaxies: Vec<(usize, usize)> = vec![];
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let mut empty_rows: Vec<usize> = vec![];
    for i in 0..lines.len() {
        if lines[i].iter().all(|x| *x == '.') {
            empty_rows.push(i);
        }
    }

    let mut empty_cols: Vec<usize> = vec![];
    for j in 0..lines[0].len() {
        let mut empty: bool = true;
        for i in 0..lines.len() {
            empty = empty && lines[i][j] == '.';
        }
        if empty {
            empty_cols.push(j);
        }
    }

    let mut result: usize = 0;
    for g1 in 0..galaxies.len() {
        for g2 in g1 + 1..galaxies.len() {
            let (mi_row, ma_row) = [galaxies[g1].0, galaxies[g2].0]
                .into_iter()
                .minmax()
                .into_option()
                .unwrap();
            let (mi_col, ma_col) = [galaxies[g1].1, galaxies[g2].1]
                .into_iter()
                .minmax()
                .into_option()
                .unwrap();
            let cnt_rows: usize = empty_rows
                .iter()
                .filter(|y| mi_row < **y && **y < ma_row)
                .count();
            let cnt_cols: usize = empty_cols
                .iter()
                .filter(|x| mi_col < **x && **x < ma_col)
                .count();

            let rows = ma_row - mi_row + cnt_rows * (EXPAND - 1);
            let cols = ma_col - mi_col + cnt_cols * (EXPAND - 1);

            println!(
                "Distance between {:?} and {:?} is {} + {} = {}",
                galaxies[g1],
                galaxies[g2],
                rows,
                cols,
                rows + cols
            );
            result += rows + cols;
        }
    }

    println!("Result: {}", result);
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
