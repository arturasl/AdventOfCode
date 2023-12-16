use std::io;
use std::thread;

fn run() {
    let lines: Vec<Vec<i64>> = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(',')
        .map(|p| p.chars().map(|ch| ch as i64).collect::<Vec<i64>>())
        .collect();

    let result: i64 = lines
        .into_iter()
        .map(|line| line.into_iter().fold(0i64, |acc, c| ((acc + c) * 17) % 256))
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
