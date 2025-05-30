use anyhow::{Ok, Result};
use itertools::Itertools;
use std::io::{self, BufRead};
use std::thread;

fn run() -> Result<()> {
    let nums: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .map(|l| Ok(l?.trim().to_owned()))
        .filter_ok(|l| !l.is_empty())
        .map(|l| l.and_then(|l| Ok(l.parse()?)))
        .collect::<Result<_>>()?;

    let result: i64 = nums
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum();

    println!("{}", result);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
