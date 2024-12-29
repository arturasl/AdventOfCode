use anyhow::{Ok, Result};
use iterr::ItErr;
use itertools::Itertools;
use std::io::{self, BufRead};
use std::thread;

fn run() -> Result<()> {
    let nums: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .map(|l| Ok(l?.trim().to_owned()))
        .lift_err(|iter| iter.filter(|l| !l.is_empty()).map(|l| Ok(l.parse()?)))
        .collect::<Result<_>>()?;

    let tripes: Vec<i64> = nums.windows(3).map(|wnd| wnd.into_iter().sum()).collect();

    let result: i64 = tripes
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
