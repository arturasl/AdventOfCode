use anyhow::{Ok, Result};
use std::io::{self, BufRead};
use std::thread;

fn run() -> Result<()> {
    println!(
        "{}",
        io::stdin()
            .lock()
            .lines()
            .map(|l| {
                l.unwrap()
                    .trim()
                    .replace(" | ", " ")
                    .split(" ")
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .take(4)
                    .filter(|s| vec![2, 4, 3, 7].contains(&s.len()))
                    .count() as i64
            })
            .sum::<i64>()
    );

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
