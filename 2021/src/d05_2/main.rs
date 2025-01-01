use ahash::AHashMap;
use anyhow::{Context, Ok, Result};
use itertools::Itertools;
use regex::Regex;
use std::io::{self, BufRead};
use std::thread;

fn run() -> Result<()> {
    let re_path = Regex::new(r"^(\d+),(\d+)->(\d+),(\d+)$").unwrap();
    let mut hits: AHashMap<(i64, i64), i64> = AHashMap::new();
    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.replace(" ", "");
        if line.is_empty() {
            continue;
        }
        let (mut x1, mut y1, x2, y2) = re_path
            .captures(&line)
            .context("matching regex")?
            .iter()
            .skip(1)
            .filter(Option::is_some)
            .map(|c| c.unwrap().as_str().parse::<i64>().unwrap())
            .collect_tuple()
            .context("Should have had 4 numbers")?;

        if y1 == y2 || x1 == x2 || (y2 - y1).abs() == (x2 - x1).abs() {
            let dy = (y2 - y1).signum();
            let dx = (x2 - x1).signum();
            while (y1, x1) != (y2 + dy, x2 + dx) {
                *hits.entry((y1, x1)).or_insert(0) += 1;
                (y1, x1) = (y1 + dy, x1 + dx);
            }
        }
    }

    println!("{}", hits.into_values().filter(|v| *v >= 2).count());

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
