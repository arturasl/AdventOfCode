use std::io;
use std::thread;

fn run() {
    let mut map: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().chars().collect::<Vec<char>>())
        .filter(|x| !x.is_empty())
        .collect();

    for col_idx in 0..map[0].len() {
        let mut swap_to: usize = 0;
        for row_idx in 0..map.len() {
            match map[row_idx][col_idx] {
                '#' => swap_to = row_idx + 1,
                '.' => {}
                'O' => {
                    map[row_idx][col_idx] = '.';
                    map[swap_to][col_idx] = 'O';
                    swap_to += 1;
                }
                _ => panic!(),
            }
        }
    }

    for row in &map {
        println!("{}", row.into_iter().collect::<String>());
    }

    let result: usize = map
        .iter()
        .enumerate()
        .map(|(i, row)| row.into_iter().filter(|x| **x == 'O').count() * (map.len() - i))
        .sum();
    println!("Result: {result}");
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
